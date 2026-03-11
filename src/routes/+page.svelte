<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
  import { listen } from "@tauri-apps/api/event";
  import type { Command, CommandsPayload, DuplicateWarning, ListItem, ReservedPhraseWarning } from "$lib/types";

  // ── State ──────────────────────────────────────────────────────────────
  let input = $state("");
  let inputEl: HTMLInputElement | undefined = $state();
  let onboardingEl: HTMLDivElement | undefined = $state();
  const appWindow = getCurrentWindow();

  // Onboarding: shown on first launch until a shortcut is chosen
  let onboarding = $state(false);
  let capturedShortcut = $state("");
  let shortcutError = $state("");

  // Command store — loaded once on mount
  let commands = $state<Command[]>([]);

  // Duplicate-command warnings from the last load / reload cycle
  let warnings = $state<DuplicateWarning[]>([]);
  let reservedWarnings = $state<ReservedPhraseWarning[]>([]);
  let warningsDismissed = $state(false);
  const totalWarnings = $derived(warnings.length + reservedWarnings.length);
  const warningVisible = $derived(totalWarnings > 0 && !warningsDismissed);

  // Active context — empty string means no context is set
  let activeContext = $state("");

  // Built-in ctx commands — always present, titles reflect current activeContext
  const builtinCommands: Command[] = $derived([
    {
      phrase: "ctx set",
      title: activeContext ? `Change context (current: "${activeContext}")` : "Set context",
      action: { type: "builtin", config: { action: "ctx_set" } },
    },
    {
      phrase: "ctx reset",
      title: "Reset context",
      action: { type: "builtin", config: { action: "ctx_reset" } },
    },
    {
      phrase: "ctx show",
      title: activeContext ? `Active context: "${activeContext}"` : "No context active",
      action: { type: "builtin", config: { action: "ctx_show" } },
    },
  ]);

  // List expansion state — populated when input exactly matches a static_list phrase
  let listItems = $state<ListItem[]>([]);
  let activeListCmd = $state<Command | null>(null);

  // ── Filtering & navigation ─────────────────────────────────────────────
  const MAX_RESULTS = 8;
  const ROW_H = 56; // px per result row

  const filtered = $derived(
    input.trim() === ""
      ? []
      : commands
          .filter(cmd => {
            const phrase = cmd.phrase.toLowerCase();
            const typed  = input.trim().toLowerCase();
            // Standard partial/substring match (discovery while typing)
            // OR param mode: user has typed the full phrase + space + param text
            return phrase.includes(typed) || typed.startsWith(phrase + " ");
          })
          .slice(0, MAX_RESULTS)
  );

  // Built-in ctx commands filtered by the current raw input (only when input starts with "ctx")
  const filteredBuiltins: Command[] = $derived(
    input.trim().toLowerCase().startsWith("ctx")
      ? builtinCommands.filter(cmd => {
          const phrase = cmd.phrase.toLowerCase();
          const typed  = input.trim().toLowerCase();
          return phrase.includes(typed) || typed.startsWith(phrase + " ");
        })
      : []
  );

  // Combined results: built-ins first, then YAML commands
  const allFiltered = $derived([...filteredBuiltins, ...filtered]);

  // True when the typed input exactly equals a static_list command phrase.
  // In this mode we show list items instead of the normal results list.
  const showingList = $derived(activeListCmd !== null && listItems.length > 0);

  let selectedIndex = $state(0);

  // Reset selection whenever the result list changes
  $effect(() => {
    void allFiltered;
    selectedIndex = 0;
  });

  // Detect exact-phrase match for static_list / dynamic_list commands and load items.
  // Returns a cleanup that cancels any in-flight debounce timer.
  $effect(() => {
    const typed = input.trim().toLowerCase();

    // ── static_list: exact match only ─────────────────────────────────
    const staticMatch = commands.find(
      cmd => cmd.action.type === "static_list" && cmd.phrase.toLowerCase() === typed
    ) ?? null;

    if (staticMatch && staticMatch.action.type === "static_list") {
      const listName = staticMatch.action.config.list;
      activeListCmd = staticMatch;
      invoke<ListItem[]>("load_list", { listName })
        .then(items => { listItems = items; selectedIndex = 0; })
        .catch(() => { listItems = []; });
      return;
    }

    // ── dynamic_list: exact match OR phrase + space + suffix ───────────
    const dynMatch = commands.find(cmd => {
      if (cmd.action.type !== "dynamic_list") return false;
      const phrase = cmd.phrase.toLowerCase();
      return typed === phrase || typed.startsWith(phrase + " ");
    }) ?? null;

    if (dynMatch && dynMatch.action.type === "dynamic_list") {
      const phrase = dynMatch.phrase.toLowerCase();
      const config = dynMatch.action.config;
      const isExact = typed === phrase;
      const suffix = typed.startsWith(phrase + " ") ? typed.slice(phrase.length + 1).trim() : "";
      const argMode = config.arg ?? "none";

      let timer: ReturnType<typeof setTimeout> | null = null;

      if (argMode === "none") {
        if (isExact) {
          activeListCmd = dynMatch;
          invoke<ListItem[]>("run_dynamic_list", { scriptName: config.script, arg: null })
            .then(items => { listItems = items; selectedIndex = 0; })
            .catch(() => { listItems = []; });
        } else {
          activeListCmd = null;
          listItems = [];
        }
      } else if (argMode === "optional") {
        activeListCmd = dynMatch;
        if (isExact) {
          // Immediate invocation — no suffix
          invoke<ListItem[]>("run_dynamic_list", { scriptName: config.script, arg: null })
            .then(items => { listItems = items; selectedIndex = 0; })
            .catch(() => { listItems = []; });
        } else {
          // Suffix present — debounce re-invocation
          timer = setTimeout(() => {
            invoke<ListItem[]>("run_dynamic_list", { scriptName: config.script, arg: suffix })
              .then(items => { listItems = items; selectedIndex = 0; })
              .catch(() => { listItems = []; });
          }, 200);
        }
      } else {
        // required: only invoke when suffix is non-empty
        if (suffix) {
          activeListCmd = dynMatch;
          timer = setTimeout(() => {
            invoke<ListItem[]>("run_dynamic_list", { scriptName: config.script, arg: suffix })
              .then(items => { listItems = items; selectedIndex = 0; })
              .catch(() => { listItems = []; });
          }, 200);
        } else {
          activeListCmd = null;
          listItems = [];
        }
      }

      return () => { if (timer !== null) clearTimeout(timer); };
    }

    // No list match
    activeListCmd = null;
    listItems = [];
  });

  // Resize window to fit current results (skip during onboarding)
  $effect(() => {
    if (onboarding) return;
    const hasQuery = input.trim() !== "";
    const WARNING_H = 40;
    const warnExtra = warningVisible ? WARNING_H : 0;
    const contentHeight = !hasQuery ? 0
      : showingList ? Math.min(listItems.length, MAX_RESULTS) * ROW_H
      : allFiltered.length === 0 ? 44          // "no results" row
      : allFiltered.length * ROW_H;
    appWindow.setSize(new LogicalSize(640, 64 + warnExtra + contentHeight));
  });

  // ── Highlight helper ──────────────────────────────────────────────────
  function highlight(phrase: string, query: string) {
    const q = query.trim().toLowerCase();
    const idx = phrase.toLowerCase().indexOf(q);
    if (idx === -1 || q === "") return { before: phrase, match: "", after: "" };
    return {
      before: phrase.slice(0, idx),
      match:  phrase.slice(idx, idx + q.length),
      after:  phrase.slice(idx + q.length),
    };
  }

  const LAUNCHER_SIZE  = new LogicalSize(640, 64);
  const ONBOARDING_SIZE = new LogicalSize(480, 240);

  // ── Helpers ────────────────────────────────────────────────────────────
  // Used for blur and programmatic hides (no focus restoration needed —
  // either the OS already moved focus elsewhere, or there is no previous app).
  function dismiss() {
    input = "";
    invoke("hide_window").catch(() => appWindow.hide());
  }

  // Used for intentional user dismissal via Escape.
  // Hides the window AND restores focus to the previously active application.
  function dismissWithFocusRestore() {
    input = "";
    invoke("dismiss_launcher").catch(() => appWindow.hide());
  }

  // Build a Tauri-compatible accelerator string from a KeyboardEvent
  function eventToShortcut(e: KeyboardEvent): string | null {
    const mods: string[] = [];
    if (e.metaKey)  mods.push("Super");
    if (e.ctrlKey)  mods.push("Control");
    if (e.altKey)   mods.push("Alt");
    if (e.shiftKey) mods.push("Shift");
    if (mods.length === 0) return null;
    const ignored = new Set(["Meta", "Control", "Alt", "Shift"]);
    if (ignored.has(e.key)) return null;
    const keyMap: Record<string, string> = {
      " ": "Space", "ArrowUp": "Up", "ArrowDown": "Down",
      "ArrowLeft": "Left", "ArrowRight": "Right",
    };
    const key = keyMap[e.key] ?? (e.key.length === 1 ? e.key.toUpperCase() : e.key);
    return [...mods, key].join("+");
  }

  // ── Onboarding key capture ─────────────────────────────────────────────
  function handleOnboardingKeydown(e: KeyboardEvent) {
    e.preventDefault();
    shortcutError = "";
    const shortcut = eventToShortcut(e);
    if (shortcut) capturedShortcut = shortcut;
  }

  async function confirmShortcut() {
    if (!capturedShortcut) return;
    try {
      await invoke("register_shortcut", { shortcut: capturedShortcut });
      localStorage.setItem("ctx_hotkey", capturedShortcut);
      onboarding = false;
      await appWindow.setSize(LAUNCHER_SIZE);
      // Load commands now that onboarding is complete
      const result = await invoke<CommandsPayload>("list_commands").catch(() => ({ commands: [], duplicates: [], reserved: [] }));
      commands = result.commands;
      warnings = result.duplicates;
      reservedWarnings = result.reserved;
      warningsDismissed = false;
      dismiss();
    } catch (err) {
      shortcutError = `Could not register shortcut: ${err}`;
      capturedShortcut = "";
    }
  }

  // ── Action execution ──────────────────────────────────────────────────
  async function executeListItem(item: ListItem) {
    const value = item.subtext ?? item.title;
    const itemAction =
      activeListCmd?.action.type === "static_list"
        ? activeListCmd.action.config.item_action
        : activeListCmd?.action.type === "dynamic_list"
        ? activeListCmd.action.config.item_action
        : undefined;
    input = "";
    if (itemAction === "paste_text") {
      await invoke("paste_text", { text: value });
    } else if (itemAction === "copy_text") {
      await invoke("copy_text", { text: value });
    } else if (itemAction === "open_url") {
      await invoke("open_url", { url: value, param: null });
      dismiss();
    } else {
      // No action configured — just dismiss
      invoke("dismiss_launcher").catch(() => appWindow.hide());
    }
  }

  async function executeCommand(cmd: Command) {
    if (cmd.action.type === "open_url") {
      // Extract any text typed after the command phrase as the param
      const phrase = cmd.phrase.toLowerCase();
      const typed  = input.trim();
      const after  = typed.toLowerCase().startsWith(phrase)
        ? typed.slice(phrase.length).trim()
        : "";
      await invoke("open_url", {
        url:   cmd.action.config.url,
        param: after !== "" ? after : null,
      });
      dismiss();
    } else if (cmd.action.type === "paste_text") {
      // Rust command handles window hide + focus restore + clipboard + keystroke.
      // We clear input here so the bar is clean when the launcher is next shown.
      input = "";
      await invoke("paste_text", { text: cmd.action.config.text });
    } else if (cmd.action.type === "copy_text") {
      // Rust command writes to clipboard and hides the launcher.
      // No paste keystroke — the user pastes manually.
      input = "";
      await invoke("copy_text", { text: cmd.action.config.text });
    } else if (cmd.action.type === "script_action") {
      const cfg = cmd.action.config;
      const phrase = cmd.phrase.toLowerCase();
      const typed  = input.trim();
      const after  = typed.toLowerCase().startsWith(phrase)
        ? typed.slice(phrase.length).trim()
        : "";

      // Determine the argument to pass based on arg mode.
      let scriptArg: string | null = null;
      if (cfg.arg === "optional" && after !== "") {
        scriptArg = after;
      } else if (cfg.arg === "required") {
        if (after === "") return; // can't execute without a required argument
        scriptArg = after;
      }
      // arg === "none" (or absent): scriptArg stays null

      const values: string[] = await invoke("run_script_action", {
        scriptName: cfg.script,
        arg: scriptArg,
      });

      if (cfg.result_action === "open_url") {
        for (const v of values) {
          await invoke("open_url", { url: v, param: null });
        }
        dismiss();
      } else {
        // paste_text or copy_text: wrap each value with prefix/suffix and join into one string.
        const text = values
          .map(v => (cfg.prefix ?? "") + v + (cfg.suffix ?? ""))
          .join("");
        input = "";
        await invoke(cfg.result_action === "paste_text" ? "paste_text" : "copy_text", { text });
      }
    } else if (cmd.action.type === "builtin") {
      const builtinAction = cmd.action.config.action;
      if (builtinAction === "ctx_set") {
        const suffix = input.trim().toLowerCase().startsWith("ctx set ")
          ? input.trim().slice("ctx set ".length).trim()
          : "";
        if (suffix) activeContext = suffix;
        input = "";
        // do NOT dismiss — launcher stays open so the user sees the updated context
      } else if (builtinAction === "ctx_reset") {
        activeContext = "";
        input = "";
        // do NOT dismiss
      } else if (builtinAction === "ctx_show") {
        input = "";
        // do NOT dismiss
      }
    }
  }

  // ── Launcher key handling ──────────────────────────────────────────────
  function handleKeydown(e: KeyboardEvent) {
    if (onboarding) return; // handled by the onboarding div
    if (e.key === "Escape") {
      e.preventDefault();
      dismissWithFocusRestore();
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      const len = showingList ? listItems.length : allFiltered.length;
      if (len > 0) selectedIndex = (selectedIndex + 1) % len;
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      const len = showingList ? listItems.length : allFiltered.length;
      if (len > 0) selectedIndex = (selectedIndex - 1 + len) % len;
    } else if (e.key === "Enter") {
      e.preventDefault();
      if (showingList) {
        const item = listItems[selectedIndex];
        if (item) executeListItem(item);
      } else {
        const cmd = allFiltered[selectedIndex];
        if (cmd) executeCommand(cmd);
      }
    }
  }

  // ── Lifecycle ─────────────────────────────────────────────────────────
  onMount(() => {
    let unlistenFocus: (() => void) | null = null;
    let unlistenReload: (() => void) | null = null;

    (async () => {
      // Migrate localStorage key from the old app name (Contexts -> Ctx)
      const legacy = localStorage.getItem("contexts_hotkey");
      if (legacy && !localStorage.getItem("ctx_hotkey")) {
        localStorage.setItem("ctx_hotkey", legacy);
        localStorage.removeItem("contexts_hotkey");
      }

      const stored = localStorage.getItem("ctx_hotkey");

      if (stored) {
        // Re-register saved shortcut, resize to launcher bar, then hide
        await invoke("register_shortcut", { shortcut: stored }).catch(() => {});
        await appWindow.setSize(LAUNCHER_SIZE);
        // Load commands from the config directory
        const result = await invoke<CommandsPayload>("list_commands").catch(() => ({ commands: [], duplicates: [], reserved: [] }));
        commands = result.commands;
        warnings = result.duplicates;
        reservedWarnings = result.reserved;
        warningsDismissed = false;
        dismiss();
      } else {
        // First launch: show onboarding at the larger size
        await appWindow.setSize(ONBOARDING_SIZE);
        onboarding = true;
        // Focus the onboarding panel so keydown events fire
        setTimeout(() => onboardingEl?.focus(), 50);
      }

      // Hide on blur, but never during onboarding
      unlistenFocus = await appWindow.onFocusChanged(({ payload: focused }) => {
        if (!focused && !onboarding) dismiss();
        if (focused && !onboarding) setTimeout(() => inputEl?.focus(), 0);
      });

      // Live-reload: backend emits this event when a YAML file changes
      unlistenReload = await listen<CommandsPayload>("commands://reloaded", (event) => {
        commands = event.payload.commands;
        warnings = event.payload.duplicates;
        reservedWarnings = event.payload.reserved;
        warningsDismissed = false; // always surface new warnings
        // If a list is currently displayed, refresh it in case its file changed
        if (activeListCmd && activeListCmd.action.type === "static_list") {
          const listName = activeListCmd.action.config.list;
          invoke<ListItem[]>("load_list", { listName })
            .then(items => { listItems = items; })
            .catch(() => { listItems = []; });
        } else if (activeListCmd && activeListCmd.action.type === "dynamic_list") {
          const config = activeListCmd.action.config;
          const typed = input.trim().toLowerCase();
          const phrase = activeListCmd.phrase.toLowerCase();
          const suffix = typed.startsWith(phrase + " ") ? typed.slice(phrase.length + 1).trim() : "";
          invoke<ListItem[]>("run_dynamic_list", { scriptName: config.script, arg: suffix || null })
            .then(items => { listItems = items; })
            .catch(() => { listItems = []; });
        }
      });
    })();

    return () => {
      unlistenFocus?.();
      unlistenReload?.();
    };
  });
</script>

<svelte:window onkeydown={handleKeydown} />

{#if onboarding}
  <!-- ── Onboarding ─────────────────────────────────────────────────────── -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    bind:this={onboardingEl}
    class="onboarding"
    role="dialog"
    aria-label="Set global shortcut"
    tabindex="-1"
    onkeydown={handleOnboardingKeydown}
  >
    <p class="ob-title">Welcome to Ctx</p>
    <p class="ob-sub">Press the key combination you want to use<br>to open the launcher from anywhere.</p>

    <div class="shortcut-preview" class:active={!!capturedShortcut}>
      {capturedShortcut || "Press a key combination…"}
    </div>

    {#if shortcutError}
      <p class="ob-error">{shortcutError}</p>
    {/if}

    <button class="ob-confirm" disabled={!capturedShortcut} onclick={confirmShortcut}>
      Confirm shortcut
    </button>
  </div>
{:else}
  <!-- ── Launcher bar ───────────────────────────────────────────────────── -->
  <div class="launcher">
    <input
      bind:this={inputEl}
      bind:value={input}
      type="text"
      placeholder="Type a command…"
      autocomplete="off"
      autocorrect="off"
      spellcheck="false"
    />

    {#if warningVisible}
      <div class="warnings-bar">
        <span class="warnings-text">
          ⚠ {totalWarnings} command{totalWarnings === 1 ? '' : 's'} ignored
        </span>
        <button class="warnings-dismiss" onclick={() => (warningsDismissed = true)} aria-label="Dismiss">&times;</button>
      </div>
    {/if}

    {#if input.trim() !== ""}
      <div class="results">
        {#if showingList}
          {#each listItems as item, i}
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <div
              class="result-row"
              class:selected={i === selectedIndex}
              onmouseenter={() => (selectedIndex = i)}
              onmousedown={(e) => { e.preventDefault(); selectedIndex = i; }}
              onclick={() => executeListItem(item)}
            >
              <span class="result-title">{item.title}</span>
              {#if item.subtext}
                <span class="result-subtext">{item.subtext}</span>
              {/if}
            </div>
          {/each}
        {:else if allFiltered.length === 0}
          <div class="no-results">No results</div>
        {:else}
          {#each allFiltered as cmd, i}
            {@const typed     = input.trim()}
            {@const builtinAction = cmd.action.type === "builtin" ? cmd.action.config.action : null}
            {@const ctxSetValue = builtinAction === "ctx_set" && typed.toLowerCase().startsWith("ctx set ") ? typed.slice("ctx set ".length).trim() : ""}
            {@const isParamMode = builtinAction === null && typed.toLowerCase().startsWith(cmd.phrase.toLowerCase() + " ")}
            {@const paramText  = isParamMode ? typed.slice(cmd.phrase.length + 1) : ""}
            {@const hl        = highlight(cmd.phrase, typed)}
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <div
              class="result-row"
              class:selected={i === selectedIndex}
              onmouseenter={() => (selectedIndex = i)}
              onmousedown={(e) => { e.preventDefault(); selectedIndex = i; }}
              onclick={() => executeCommand(cmd)}
            >
              <span class="result-title">{cmd.title}</span>
              <span class="result-subtext">
                {#if ctxSetValue}
                  → set context to "{ctxSetValue}"
                {:else if isParamMode}
                  {cmd.phrase}<span class="param-hint"> → {paramText}</span>
                {:else}
                  {hl.before}<mark>{hl.match}</mark>{hl.after}
                {/if}
              </span>
            </div>
          {/each}
        {/if}
      </div>
    {/if}
  </div>
{/if}

<style>
  :global(*) { box-sizing: border-box; }

  :global(body) {
    margin: 0;
    background: transparent;
    overflow: hidden;
    user-select: none;
    -webkit-user-select: none;
  }

  /* ── Launcher bar ────────────────────────────────────────────────────── */
  .launcher {
    background: rgba(28, 28, 30, 0.95);
    border-radius: 12px;
    box-shadow: 0 24px 64px rgba(0,0,0,.6), 0 0 0 1px rgba(255,255,255,.08);
    overflow: hidden;
  }

  input {
    width: 100%;
    background: transparent;
    border: none;
    color: #f5f5f7;
    font-size: 18px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    padding: 20px 24px;
    outline: none;
    caret-color: #0a84ff;
  }

  input::placeholder { color: rgba(245,245,247,.35); }

  /* ── Onboarding ──────────────────────────────────────────────────────── */
  .onboarding {
    background: rgba(28, 28, 30, 0.97);
    border-radius: 12px;
    box-shadow: 0 24px 64px rgba(0,0,0,.6), 0 0 0 1px rgba(255,255,255,.08);
    padding: 28px 28px 24px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    outline: none;
    height: 100%;
  }

  .ob-title {
    margin: 0;
    color: #f5f5f7;
    font-size: 17px;
    font-weight: 600;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  }

  .ob-sub {
    margin: 0;
    color: rgba(245,245,247,.5);
    font-size: 13px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    text-align: center;
    line-height: 1.5;
  }

  .shortcut-preview {
    background: rgba(255,255,255,.07);
    border: 1px solid rgba(255,255,255,.12);
    border-radius: 8px;
    padding: 10px 20px;
    color: rgba(245,245,247,.35);
    font-size: 15px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    min-width: 160px;
    text-align: center;
    transition: color .15s, border-color .15s;
  }

  .shortcut-preview.active {
    color: #f5f5f7;
    border-color: rgba(10,132,255,.6);
  }

  .ob-error {
    margin: 0;
    color: #ff453a;
    font-size: 12px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  }

  .ob-confirm {
    background: #0a84ff;
    color: #fff;
    border: none;
    border-radius: 8px;
    padding: 9px 24px;
    font-size: 14px;
    font-weight: 500;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    cursor: pointer;
    transition: opacity .15s;
  }

  .ob-confirm:disabled { opacity: .35; cursor: default; }

  /* ── Results list ────────────────────────────────────────────────────── */
  .results {
    border-top: 1px solid rgba(255,255,255,.07);
    padding: 4px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .result-row {
    display: flex;
    flex-direction: column;
    gap: 3px;
    padding: 10px 16px;
    border-radius: 8px;
    cursor: default;
    transition: background .1s;
  }

  .result-row.selected {
    background: rgba(255,255,255,.09);
  }

  .result-title {
    color: #f5f5f7;
    font-size: 14px;
    font-weight: 500;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .result-subtext {
    color: rgba(245,245,247,.4);
    font-size: 12px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .result-subtext mark {
    background: transparent;
    color: #0a84ff;
    font-weight: 600;
  }

  .param-hint {
    color: #0a84ff;
    font-weight: 500;
  }

  .no-results {
    padding: 12px 16px;
    color: rgba(245,245,247,.3);
    font-size: 13px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    text-align: center;
  }

  /* ── Duplicate warnings bar ─────────────────────────────────────────── */
  .warnings-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 16px;
    height: 40px;
    background: rgba(255, 159, 10, 0.12);
    border-top: 1px solid rgba(255, 159, 10, 0.25);
  }

  .warnings-text {
    color: #ff9f0a;
    font-size: 12px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  }

  .warnings-dismiss {
    background: none;
    border: none;
    color: rgba(255, 159, 10, 0.6);
    font-size: 16px;
    line-height: 1;
    cursor: pointer;
    padding: 2px 4px;
    border-radius: 4px;
    transition: color .15s;
  }

  .warnings-dismiss:hover { color: #ff9f0a; }
</style>

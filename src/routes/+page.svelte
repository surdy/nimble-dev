<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
  import { listen } from "@tauri-apps/api/event";
  import type { Command, CommandsPayload, DuplicateWarning } from "$lib/types";

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
  let warningsDismissed = $state(false);
  const warningVisible = $derived(warnings.length > 0 && !warningsDismissed);

  // ── Filtering & navigation ─────────────────────────────────────────────
  const MAX_RESULTS = 8;
  const ROW_H = 56; // px per result row

  const filtered = $derived(
    input.trim() === ""
      ? []
      : commands
          .filter(cmd => cmd.phrase.toLowerCase().includes(input.trim().toLowerCase()))
          .slice(0, MAX_RESULTS)
  );

  let selectedIndex = $state(0);

  // Reset selection whenever the result list changes
  $effect(() => {
    void filtered;
    selectedIndex = 0;
  });

  // Resize window to fit current results (skip during onboarding)
  $effect(() => {
    if (onboarding) return;
    const hasQuery = input.trim() !== "";
    const WARNING_H = 40;
    const warnExtra = warningVisible ? WARNING_H : 0;
    const contentHeight = !hasQuery ? 0
      : filtered.length === 0 ? 44          // "no results" row
      : filtered.length * ROW_H;
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
      localStorage.setItem("contexts_hotkey", capturedShortcut);
      onboarding = false;
      await appWindow.setSize(LAUNCHER_SIZE);
      dismiss();
    } catch (err) {
      shortcutError = `Could not register shortcut: ${err}`;
      capturedShortcut = "";
    }
  }

  // ── Action execution ──────────────────────────────────────────────────
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
      if (filtered.length > 0) selectedIndex = (selectedIndex + 1) % filtered.length;
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      if (filtered.length > 0) selectedIndex = (selectedIndex - 1 + filtered.length) % filtered.length;
    } else if (e.key === "Enter") {
      e.preventDefault();
      const cmd = filtered[selectedIndex];
      if (cmd) executeCommand(cmd);
    }
  }

  // ── Lifecycle ─────────────────────────────────────────────────────────
  onMount(() => {
    let unlistenFocus: (() => void) | null = null;
    let unlistenReload: (() => void) | null = null;

    (async () => {
      const stored = localStorage.getItem("contexts_hotkey");

      if (stored) {
        // Re-register saved shortcut, resize to launcher bar, then hide
        await invoke("register_shortcut", { shortcut: stored }).catch(() => {});
        await appWindow.setSize(LAUNCHER_SIZE);
        // Load commands from the config directory
        const result = await invoke<CommandsPayload>("list_commands").catch(() => ({ commands: [], duplicates: [] }));
        commands = result.commands;
        warnings = result.duplicates;
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
        warningsDismissed = false; // always surface new warnings
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
    <p class="ob-title">Welcome to Contexts</p>
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
          ⚠ {warnings.length} duplicate command{warnings.length === 1 ? '' : 's'} ignored
        </span>
        <button class="warnings-dismiss" onclick={() => (warningsDismissed = true)} aria-label="Dismiss">&times;</button>
      </div>
    {/if}

    {#if input.trim() !== ""}
      <div class="results">
        {#if filtered.length === 0}
          <div class="no-results">No results</div>
        {:else}
          {#each filtered as cmd, i}
            {@const hl = highlight(cmd.phrase, input)}
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div
              class="result-row"
              class:selected={i === selectedIndex}
              onmouseenter={() => (selectedIndex = i)}
              onmousedown={(e) => { e.preventDefault(); selectedIndex = i; }}
            >
              <span class="result-title">{cmd.title}</span>
              <span class="result-subtext">{hl.before}<mark>{hl.match}</mark>{hl.after}</span>
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

<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";

  // ── State ──────────────────────────────────────────────────────────────
  let input = $state("");
  let inputEl: HTMLInputElement | undefined = $state();
  let onboardingEl: HTMLDivElement | undefined = $state();
  const appWindow = getCurrentWindow();

  // Onboarding: shown on first launch until a shortcut is chosen
  let onboarding = $state(false);
  let capturedShortcut = $state("");
  let shortcutError = $state("");

  const LAUNCHER_SIZE  = new LogicalSize(640, 64);
  const ONBOARDING_SIZE = new LogicalSize(480, 240);

  // ── Helpers ────────────────────────────────────────────────────────────
  function dismiss() {
    input = "";
    invoke("hide_window").catch(() => appWindow.hide());
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

  // ── Launcher key handling ──────────────────────────────────────────────
  function handleKeydown(e: KeyboardEvent) {
    if (onboarding) return; // handled by the onboarding div
    if (e.key === "Escape") {
      e.preventDefault();
      dismiss();
    }
  }

  // ── Lifecycle ─────────────────────────────────────────────────────────
  onMount(() => {
    let unlistenFn: (() => void) | null = null;

    (async () => {
      const stored = localStorage.getItem("contexts_hotkey");

      if (stored) {
        // Re-register saved shortcut, resize to launcher bar, then hide
        await invoke("register_shortcut", { shortcut: stored }).catch(() => {});
        await appWindow.setSize(LAUNCHER_SIZE);
        dismiss();
      } else {
        // First launch: show onboarding at the larger size
        await appWindow.setSize(ONBOARDING_SIZE);
        onboarding = true;
        // Focus the onboarding panel so keydown events fire
        setTimeout(() => onboardingEl?.focus(), 50);
      }

      // Hide on blur, but never during onboarding
      const unlistenPromise = appWindow.onFocusChanged(({ payload: focused }) => {
        if (!focused && !onboarding) dismiss();
      });
      unlistenFn = await unlistenPromise;
    })();

    return () => { unlistenFn?.(); };
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
      onkeydown={handleKeydown}
    />
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
</style>

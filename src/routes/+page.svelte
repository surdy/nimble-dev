<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  let input = $state("");
  let inputEl: HTMLInputElement;
  const appWindow = getCurrentWindow();

  function dismiss() {
    invoke("close_window").catch(() => appWindow.close());
  }

  onMount(() => {
    inputEl.focus();

    // Close the launcher when it loses focus (user clicked elsewhere)
    const unlistenPromise = appWindow.onFocusChanged(({ payload: focused }) => {
      if (!focused) dismiss();
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      e.preventDefault();
      dismiss();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="launcher">
  <input
    bind:this={inputEl}
    bind:value={input}
    type="text"
    placeholder="Type a command..."
    autocomplete="off"
    autocorrect="off"
    spellcheck="false"
    onkeydown={handleKeydown}
  />
</div>

<style>
  :global(*) {
    box-sizing: border-box;
  }

  :global(body) {
    margin: 0;
    background: transparent;
    overflow: hidden;
    user-select: none;
    -webkit-user-select: none;
  }

  .launcher {
    background: rgba(28, 28, 30, 0.95);
    border-radius: 12px;
    box-shadow:
      0 24px 64px rgba(0, 0, 0, 0.6),
      0 0 0 1px rgba(255, 255, 255, 0.08);
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

  input::placeholder {
    color: rgba(245, 245, 247, 0.35);
  }
</style>

# Development Plan

Iterative implementation plan for Contexts Launcher, from bare minimum working shell to full feature set. Each stage produces a working, committable increment.

---

## Stage 1 — Launcher Window Shell ✅

**Goal:** A minimal window that can be opened and closed, runs persistently in the background, and is summoned via a user-chosen global hotkey.

### Tasks
- Configure Tauri window: frameless, transparent, always-on-top, centered, fixed size
- Enable `macOSPrivateApi` for native window transparency
- First-run onboarding screen: prompt user to press a key combination; register it as the global hotkey
- Save the chosen hotkey to `localStorage`; re-register it automatically on every subsequent launch
- App runs in the background after onboarding — window is **hidden** (not closed) on dismiss
- Global hotkey toggles the window: press once to show, press again to hide
- Dismiss the launcher on Escape key or when the window loses focus
- Resize the window dynamically: 480×240 for onboarding, 640×64 for the launcher bar

### Done when ✅
- First launch shows onboarding; user picks a shortcut; app hides and stays running
- The chosen shortcut summons and dismisses the launcher from any application
- Escape and focus-loss also hide the window

---

## Stage 2 — Command Data Model ✅

**Goal:** Define what a command looks like in code, load commands from a user-editable YAML file in a platform-appropriate config directory, and expose them to the frontend via a Tauri command.

### Tasks
- Define the command schema:
  ```
  {
    phrase: string          // the multi-word command phrase (e.g. "open google")
    title: string           // human-readable description shown as the result title
    action: {
      type: "open_url" | "paste_text"
      config: { … }         // action-specific fields
    }
  }
  ```
- User commands are stored as individual YAML files in the platform config directory (one command per file):
  - **macOS**: `~/Library/Application Support/com.contexts.launcher/`
  - **Linux**: `$XDG_CONFIG_HOME/com.contexts.launcher/` (falls back to `~/.config/com.contexts.launcher/`)
  - **Windows**: `%APPDATA%\com.contexts.launcher\`
  - Files are discovered **recursively** — commands can be organised into subdirectories
- Seed an `examples/` subdirectory with 5 individual command files on first launch if no YAML files exist
- Parse the YAML file into typed Rust structs (`Command`, `Action`, `OpenUrlConfig`, `PasteTextConfig`)
- Expose the loaded command list to the frontend via a `list_commands` Tauri command

### Done when ✅
- `list_commands` Tauri command returns the parsed list of commands to the frontend
- Config directory and `commands.yaml` are created automatically on first run

---

## Stage 3 — Partial Matching & Results UI ✅

**Goal:** As the user types, filter commands in real time and display matching results.

### Tasks
- Implement partial matching: a typed string matches any command whose phrase contains it as a contiguous substring (case-insensitive); up to 8 results shown
- Display each match as a result row:
  - **Title** (main line): the command's `title` field
  - **Subtext** (secondary line): the full command phrase
- Highlight the matching portion of the phrase in the subtext (blue, bold)
- Keyboard navigation: Up/Down arrows move selection; Enter reserved for Stage 4/5
- Show a "No results" row when input is non-empty but nothing matches
- Window resizes dynamically: 64px (empty) → 64 + n×56px (results) → 64 + 44px (no results)
- Input refocused automatically when the launcher window regains focus

### Done when ✅
- Typing partial phrases filters the list live; keyboard and mouse selection work; window grows/shrinks with results

---

## Stage 4 — Action: Open URL ✅

**Goal:** Executing a selected command with type `open_url` opens a URL in the default browser.

### Tasks
- Implement the `open_url` Tauri command in Rust using `tauri-plugin-opener`
- Support an optional `param` variable: text typed after the command phrase is URL-encoded and substituted for `{param}` in the configured URL
  - Example: phrase `"search google"`, URL `"https://google.com/search?q={param}"`, user types `"search google rust programming"` → opens `https://google.com/search?q=rust+programming`
- Validate scheme before opening — only `http://` and `https://` are accepted; anything else returns an error
- Frontend `executeCommand()` helper extracts the param from the typed input, invokes `open_url`, then dismisses the launcher
- Enter key on a selected result triggers execution

### Done when ✅
- Selecting an `open_url` command opens the correct URL in the browser; param substitution works; window hides after execution

---

## Stage 5 — Action: Paste Text ✅

**Goal:** Executing a selected command with type `paste_text` pastes a predefined string into the app that had focus before the launcher was invoked.

### Tasks
- Track the previously focused application's PID in `PreviousApp(Mutex<Option<i32>>)` state, captured
  in the global-shortcut handler and tray Show/Hide handler before the launcher window is shown
- Implement `paste_text` Tauri command:
  1. Validate text (plain text only; reject NUL bytes)
  2. Hide launcher window and update tray label
  3. Restore focus to previous app via `NSRunningApplication.activateWithOptions` (macOS)
  4. Sleep 80 ms to let focus transfer complete
  5. Write text to clipboard via `pbcopy` subprocess (macOS); `arboard` for other platforms (future)
  6. Simulate Cmd+V (macOS) or Ctrl+V via `enigo 0.2`
- Frontend `executeCommand()` calls `invoke("paste_text", { text })` and clears input
- **Requires macOS Accessibility permission** for key simulation (standard for all launcher apps)

### Done when ✅
- Executing a `paste_text` command pastes the configured text into the previously active application

---

## Stage 6 — Global Hotkey ✅

**Goal:** The user can open and dismiss the launcher from any application using a keyboard shortcut.

### Tasks
- Global hotkey registered via `tauri-plugin-global-shortcut` (user-chosen during onboarding, re-registered on every launch from `localStorage`) ✅
- Pressing the hotkey while hidden: capture frontmost app PID → show and focus launcher ✅
- Pressing the hotkey while visible: hide launcher → restore focus to previous app ✅
- Pressing Escape: hide launcher via `dismiss_launcher` command → restore focus to previous app ✅
- Blur dismiss (window loses focus because user clicked elsewhere): `hide_window` only — OS already transferred focus, no explicit restore needed ✅
- Tray Show/Hide: captures previous app on show; restores focus on hide ✅
- Global shortcut is unregistered cleanly on app quit by `tauri-plugin-global-shortcut` automatically ✅

### Done when ✅
- The launcher is summoned and dismissed system-wide; focus always returns to the correct app

---

## Stage 7 — Live Config Reload

**Goal:** Commands hot-reload when any YAML file in the config directory tree is added, changed, or removed — without requiring a restart.

### Tasks
- Watch the entire config directory **recursively** for file-system events (use `notify` crate with a debounce)
- On any `.yaml`/`.yml` change (create / modify / delete / rename), re-run `load_from_dir` and emit a Tauri event (`commands://reloaded`) carrying the new command list to the frontend
- Frontend listens for `commands://reloaded` and updates the `commands` state in place
- Document the config directory location and the per-file format in `docs/`

### Done when
- Adding, editing, or deleting any command YAML file causes the launcher's results to update without restarting the app

---

## Stage 8 — Script Extensions

**Goal:** Commands can be associated with external scripts that process input and return results for the launcher to act on.

### Tasks
- Extend the command schema with a `script` action type:
  ```
  {
    type: "script"
    config: {
      executable: string    // path to the script/binary
      args?: string[]       // static arguments
    }
  }
  ```
- When a script command is selected (or as the user types a param), invoke the executable:
  - Pass any user-supplied parameter as a command-line argument
  - Capture stdout; stderr is logged but not shown
  - Enforce a timeout (e.g. 5 seconds)
- Parse the output: plain text → single result; valid JSON array → list of results
- Each JSON result must conform to:
  ```json
  {
    "title": "string",
    "subtext": "string",
    "action": { "type": "open_url" | "paste_text", "config": { ... } }
  }
  ```
- **Security boundaries (non-negotiable):**
  - Scripts may only return data; they cannot trigger actions directly
  - Validate and sanitise all script output before rendering or acting on it
  - Only allow `open_url` (http/https only) and `paste_text` as result actions — no arbitrary shell commands
  - Never execute scripts with elevated privileges
- Show results from the script in the launcher UI; user selects one; launcher executes the associated built-in action

### Done when
- A user can define a script command, the script returns JSON results, and selecting a result triggers the correct built-in action

---

## Summary Table

| Stage | Feature | Deliverable |
|-------|---------|-------------|
| 1 | Launcher window shell | Frameless window with input, closes on Escape |
| 2 | Command data model | Typed schema + YAML file loading from platform config dir |
| 3 | Partial matching & results UI | Live filtering, keyboard navigation, title/subtext display |
| 4 | Action: Open URL | Opens URLs in browser, supports `{param}` substitution |
| 5 | Action: Paste Text | Pastes text into previously focused application |
| 6 | Global hotkey | System-wide shortcut to summon/dismiss launcher |
| 7 | Live config reload | Hot-reload commands when `commands.yaml` is edited |
| 8 | Script extensions | External scripts return structured results; launcher executes built-in actions |

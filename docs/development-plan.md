# Development Plan

Iterative implementation plan for Ctx, from bare minimum working shell to full feature set. Each stage produces a working, committable increment.

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
  - **macOS**: `~/Library/Application Support/com.ctx.launcher/`
  - **Linux**: `$XDG_CONFIG_HOME/com.ctx.launcher/` (falls back to `~/.config/com.ctx.launcher/`)
  - **Windows**: `%APPDATA%\com.ctx.launcher\`
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

## Stage 7 — Live Config Reload ✅

**Goal:** Commands hot-reload when any YAML file in the config directory tree is added, changed, or removed — without requiring a restart.

### Tasks
- Watch the entire config directory **recursively** for file-system events (use `notify` crate with a debounce)
- On any `.yaml`/`.yml` change (create / modify / delete / rename), re-run `load_from_dir` and emit a Tauri event (`commands://reloaded`) carrying the new command list to the frontend
- Frontend listens for `commands://reloaded` and updates the `commands` state in place
- Document the config directory location and the per-file format in `docs/`

### Done when
- Adding, editing, or deleting any command YAML file causes the launcher's results to update without restarting the app

---

## Stage 8 — Bug Fixes ✅

**Goal:** Address reported issues discovered during real-world use of stages 1–7.

### Fix 1 — Deleting a command file does not update the launcher

**Root cause:** `is_yaml_event()` in `watcher.rs` filters events by checking whether the affected path has a `.yaml`/`.yml` extension. On macOS, FSEvents can report a file-deletion event with the **parent directory path** rather than the deleted file's path — so the extension check fails and no reload is triggered.

**Fix:** Relax the extension check for `EventKind::Remove` events. Any removal inside the watched config directory should trigger a reload; since the file is already gone it cannot be inspected, and the config dir is low-churn enough that false-positive reloads are harmless.

### Done when ✅
- Deleting a `.yaml` command file causes the command to disappear from the launcher immediately (within the 300 ms debounce window), without restarting the app

### Fix 2 — Launcher window drifts lower on screen after each command execution

**Root cause:** The frontend `$effect` calls `setSize` (to shrink the window back to 64 px after results disappear) on a window that is already hidden. On macOS, `setSize` on a hidden window anchors from the **bottom-left corner** (the native coordinate origin), so shrinking the height by `Δ` moves the window's top-left position **down** by `Δ` pixels. The drift accumulates with every invocation.

**Fix:** Before every `window.show()` call (global-hotkey path, tray path, and `show_window` command), first call `window.set_size(640×64)` then `window.center()`. This resets any accumulated drift and places the launcher in the center of the screen on each invocation.

### Done when ✅
- Invoking the launcher repeatedly after running commands always shows it in the same centered position, never drifting

---

## Stage 9 — Enhancements ✅

**Goal:** Quality-of-life improvements to the core command system, added one at a time.

### Enhancement 1 — Enable / disable commands

Users can set `enabled: false` in a command YAML file to temporarily disable the command without deleting the file. Disabled commands are filtered out by the Rust loader and never sent to the frontend — they do not appear in the results list and cannot be executed. All commands are enabled by default (omitting the `enabled` field is equivalent to `enabled: true`).

**Schema change** (`commands.rs` and `types.ts`):
```yaml
phrase: open reddit
title: Open Reddit
enabled: false          # omit or set true to enable
action:
  type: open_url
  config:
    url: https://www.reddit.com
```

**Implementation:**
- Add `#[serde(default = "default_true")] pub enabled: bool` to the `Command` struct in Rust
- Filter disabled commands in `load_from_dir` before returning the list
- The `enabled` field is purely a load-time gate; it is never forwarded to the frontend

### Done when ✅
- A command with `enabled: false` does not appear in the launcher
- A command that omits `enabled` (or sets it to `true`) behaves exactly as before
- Live reload respects the flag: toggling `enabled` in a YAML file updates the list immediately

---

## Stage 10 — App Rename: Contexts → Ctx ✅

**Goal:** Rename the application from *Contexts* to *Ctx* everywhere — product name, bundle identifier, config directory, localStorage keys, log prefixes, and all documentation. This is a breaking change to the config directory path; existing users must migrate their command files.

### Changes

| Location | Before | After |
|----------|--------|-------|
| Product name (`tauri.conf.json`) | `Contexts` | `Ctx` |
| Bundle identifier | `com.contexts.launcher` | `com.ctx.launcher` |
| Cargo package name | `contexts-launcher` | `ctx-launcher` |
| Cargo lib name | `contexts_launcher_lib` | `ctx_launcher_lib` |
| npm package name | `contexts-launcher` | `ctx-launcher` |
| Config dir (macOS) | `~/Library/Application Support/com.contexts.launcher/` | `~/Library/Application Support/com.ctx.launcher/` |
| localStorage hotkey key | `contexts_hotkey` | `ctx_hotkey` |
| Log prefix | `[contexts]` | `[ctx]` |
| Onboarding title | `Welcome to Contexts` | `Welcome to Ctx` |
| Tray menu items | `Contexts vX.Y.Z` / `Quit Contexts` | `Ctx vX.Y.Z` / `Quit Ctx` |

### Migration note
Because the bundle identifier changes, Tauri will use a new config directory. Users upgrading from the *Contexts* build must manually move their command files:
```bash
mv ~/Library/Application\ Support/com.contexts.launcher \
   ~/Library/Application\ Support/com.ctx.launcher
```

### Done when ✅
- App builds and runs under the new name and identifier
- Config directory is `com.ctx.launcher`; command files load correctly
- No references to the old name remain in source, config, or docs

---

## Stage 11 — Documentation ✅

**Goal:** Write comprehensive user-facing documentation covering how to use the launcher, configure commands, and get the most out of the built-in functionality.

### Deliverables

| File | Content |
|------|---------|
| `docs/using/first-run.md` | Onboarding walkthrough and shortcut setup |
| `docs/using/basic-functionality.md` | Core actions: Open URL and Paste Text |
| `docs/using/tips-and-tricks.md` | Deep links, web search via param passing, frequently-pasted text examples |
| `docs/using/configuring-commands.md` | YAML schema, enable/disable, directory layout, live reload |
| `docs/using/duplicate-commands.md` | How duplicate phrases are detected, resolved, and surfaced |

### Done when ✅
- All five docs are written, accurate, and cross-referenced
- Configuration reference covers the full YAML schema and all valid action types

---

## Stage 12 — Action: Copy Text & Config Directory Restructure ✅

**Goal:** Add the `copy_text` built-in action, and reorganise the config directory so that command files live in a dedicated `commands/` subdirectory, leaving room for other data types in the config root in future stages.

### Tasks

#### Copy Text action
- Add `CopyTextConfig { text: String }` struct and `CopyText(CopyTextConfig)` variant to the `Action` enum in `commands.rs`
- Add corresponding `CopyTextConfig` interface and `copy_text` action type to `types.ts`
- Implement the `copy_text` Tauri command in Rust:
  1. Validate text (plain text only; reject NUL bytes)
  2. Write text to clipboard via `pbcopy` subprocess (macOS); `arboard` for other platforms (future)
  3. Hide the launcher window (no focus restoration needed — the user will paste manually)
- Frontend `executeCommand()` handles the `copy_text` case: invoke `copy_text`, clear input, dismiss
- Add `examples/copy-email.yaml` seed file; update `docs/using/basic-functionality.md` with Copy Text section

#### Config directory restructure
- Move command loading (`list_commands`) and file watcher to use `config_dir/commands/` instead of `config_dir` directly
- Migrate existing live command files: `examples/` → `commands/examples/`
- Add `docs/using/config-directory.md` documenting the root layout and each subdirectory
- Update `configuring-commands.md` and README to reference the new `commands/` path
- Add copilot instruction to keep `docs/using/config-directory.md` current as new subdirs are added

### Done when
- Executing a `copy_text` command writes the configured text to the clipboard and dismisses the launcher
- A command with `paste_text` still pastes automatically; `copy_text` only copies
- Command files are loaded from and watched in `config_dir/commands/`
- `docs/using/config-directory.md` exists and accurately describes the layout

---

## Stage 13 — Backend Testing

**Goal:** Add automated tests for the Rust backend covering the logic most likely to regress: YAML parsing, deduplication, URL validation, `{param}` encoding, and text sanitisation. Tests run with `cargo test` — no Tauri runtime required.

### Test modules

#### `commands.rs` — YAML loading & deduplication
- Valid single-command YAML file parses correctly into a `Command` struct
- All three action types (`open_url`, `paste_text`, `copy_text`) deserialise correctly
- `enabled: false` command is filtered out by `load_from_dir`
- Command that omits `enabled` defaults to `true`
- Two files with the same phrase produce one command and one `DuplicateWarning`; the older file wins
- Malformed YAML in one file does not prevent other files from loading
- Phrase starting with `ctx` is rejected if reserved-namespace enforcement is added

#### `lib.rs` — URL validation & param encoding
- Plain string with no scheme is rejected
- `http://` and `https://` URLs are accepted
- Deep link schemes (`slack://`, `obsidian://`, `mailto:`) are accepted
- `{param}` is replaced with URL-encoded text: spaces become `+`, special chars become `%XX`
- URL with no `{param}` and a `Some(param)` value is opened unchanged (param is ignored)
- URL with `{param}` and `param = None` is opened with the literal `{param}` in place

#### `lib.rs` — text sanitisation
- Text containing a NUL byte (`\0`) returns `Err` from `paste_text` / `copy_text`
- Plain text (no NUL) returns `Ok`

### Done when
- `cargo test` in `src-tauri/` passes with all tests green
- No Tauri `AppHandle` / `Window` mocking required (pure-logic functions only)

---

## Stage 14 — Script Extensions

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

## Stage 15 — Contexts: Core Model & Built-in Commands

**Goal:** Introduce the concept of a *context* — a phrase prefix that is silently prepended to the user's input, letting them reach a group of related commands with less typing. This stage covers the data model, the built-in commands that manage context, and the reserved `ctx` namespace.

### Concepts

A **context** is a string (possibly multi-word, e.g. `"reddit"`) stored in frontend state. When a context is active, the launcher behaves as if the user typed `"<context> <input>"` instead of just `"<input>"`. The default context is empty, which restores full-command behaviour.

**Built-in commands** are synthetic commands generated by the launcher itself, not loaded from YAML files. They are identified by the reserved prefix `ctx` (case-insensitive). User-defined YAML commands whose `phrase` starts with `ctx` (followed by a space or end-of-string) must be rejected at load time with a warning.

### Tasks

#### Reserved namespace
- At load time, reject any parsed `Command` whose phrase starts with `ctx` (case-insensitive, optionally followed by a space). Emit a warning alongside existing duplicate warnings.
- Add a `reserved_phrase` variant to the warning type so the UI can surface it distinctly if needed (or reuse the existing warning bar).

#### Built-in `ctx` commands
The following built-in commands are always available, regardless of loaded YAML:

| Phrase | Title | Effect |
|--------|-------|--------|
| `ctx set <phrase>` | Set context to "&lt;phrase&gt;" | Sets active context to `<phrase>` |
| `ctx reset` | Reset context | Clears active context (returns to default) |
| `ctx show` | Current context: "&lt;phrase&gt;" | No-op action; just shows the current context |

Built-in commands appear in the filtered results list like regular commands. Selecting `ctx set …` or `ctx reset` executes immediately without opening a URL or pasting text — the launcher stays visible after context changes (so the user can immediately start typing narrowed commands).

#### Matching with active context
When a context `C` is active, a user's raw input `I` is matched against command phrases as if the input were `C + " " + I`. Regular commands are filtered; built-in `ctx` commands are always matchable regardless of context.

### Done when
- Typing `ctx set reddit` and pressing Enter sets the active context to `"reddit"`
- Typing `ctx reset` and pressing Enter clears the context
- With context `"reddit"` active, typing `"tech"` matches a command with phrase `"reddit tech"`
- YAML commands whose phrase starts with `ctx` are rejected at load time with a warning
- Built-in commands appear in the results list and are keyboard-navigable

---

## Stage 16 — Contexts: UI Indicators & Tray Integration

**Goal:** Make the active context visible at all times — both inside the launcher window and in the system tray — so the user always knows which context is in effect.

### Tasks

#### Launcher input area
- When a context is active, display a non-editable **context chip** to the left of the text input inside the launcher bar (e.g. a small pill badge showing `reddit ×`). The chip is styled distinctly (e.g. blue background, rounded).
- The `×` on the chip clears the context immediately (same as `ctx reset`).
- The input placeholder changes to `"Type a command…"` when no context is set, and `"Narrow commands…"` when a context is active.
- The chip is purely cosmetic / a shortcut — it does not affect the input field's value.

#### Window height
- Account for the chip row height in the dynamic window-resize `$effect` when a context is active and the chip is shown.

#### System tray
- When a context is active, append the context name to the tray tooltip / app-info menu item, e.g. `"Ctx — reddit"`.
- When no context is active, show the default `"Ctx vX.Y.Z"` label.
- The tray label update must happen on the same thread that manages the `TrayMenuState`.

#### Persistence
- Active context is stored in `localStorage` under the key `ctx_active_context` so it survives launcher restarts.
- On mount, restore the saved context (if any) before the first filter pass.

### Done when
- The active context is shown as a chip in the launcher bar with a clear button
- The system tray info item reflects the active context name
- The context persists across launcher restarts (hide/show cycles and full quit-relaunch)

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
| 7 ✅ | Live config reload | Hot-reload commands when `commands.yaml` is edited |
| 8 ✅ | Bug fixes | Fix issues found during real-world use of stages 1–7 |
| 9 ✅ | Enhancements | Quality-of-life improvements to the core command system |
| 10 ✅ | App rename | Rename Contexts → Ctx; update identifier, config dir, localStorage keys |
| 11 ✅ | Documentation | User-facing docs: first run, core actions, tips & tricks, configuration, duplicates |
| 12 ✅ | Action: Copy Text & Config Directory Restructure | `copy_text` action; commands moved to `commands/` subdir |
| 13 | Backend testing | `cargo test` suite: YAML parsing, dedup, URL validation, param encoding, text sanitisation |
| 14 | Script extensions | External scripts return structured results; launcher executes built-in actions |
| 15 | Contexts: core model | Reserved `ctx` namespace, built-in set/reset commands, context-aware matching |
| 16 | Contexts: UI & tray | Context chip in launcher bar, tray label, localStorage persistence |

# Development Plan

Iterative implementation plan for Context Actions, from bare minimum working shell to full feature set. Each stage produces a working, committable increment.

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
| 10 ✅ | App rename | Rename Contexts → Context Actions; update identifier, config dir, localStorage keys |
| 11 ✅ | Documentation | User-facing docs: first run, core actions, tips & tricks, configuration, duplicates |
| 12 ✅ | Action: Copy Text & Config Directory Restructure | `copy_text` action; commands moved to `commands/` subdir |
| 13 ✅ | Backend testing | `cargo test` suite: YAML parsing, dedup, URL validation, param encoding, text sanitisation |
| 14 ✅ | Action: Static List | Keyword-triggered inline list expansion from `lists/` config subdir |
| 15 ✅ | Action: Dynamic List | Script-backed dynamic list; three argument modes (`none` / `optional` / `required`) |
| 16 ✅ | Docs restructure & cleanup | Per-action pages in `basic/` and `advanced/`; onboarding gaps filled; stale content removed |
| 17 ✅ | Action: Script Action | On-Enter script execution; result applied via `open_url`, `paste_text`, or `copy_text`; optional arg; prefix/suffix for list results |
| 18 ✅ | Example config directory | `example-config/` in repo root — one file per action type, ready to copy |
| 19a ✅ | Contexts: reserved namespace | Rust backend rejects `ctx`-prefixed YAML phrases; `ReservedPhraseWarning` added to load result |
| 19b ✅ | Contexts: state & built-in commands | `activeContext` state; `ctx set / reset / show` built-in commands in frontend results list |
| 19c ✅ | Contexts: context-aware matching | `effectiveInput = raw_input + " " + context`; filtering, list triggers, and param extraction use effective input |
| 20 ✅ | Contexts: UI & tray | Context chip in launcher bar, tray label, localStorage persistence |
| 21 ✅ | Settings file | `settings.yaml` for `hotkey`, `show_context_chip`, `allow_duplicates`; hotkey migrated from localStorage |
| 22 ✅ | Housekeeping | App rename, MIT licence, roadmap, motivation doc, docs cleanup |
| 23 ✅ | Cross-platform clipboard | `arboard` crate for Linux & Windows; `pbcopy` retained on macOS |
| 24 ✅ | Linux focus tracking | `xdotool` capture/restore on X11; Wayland graceful fallback |
| 25 ✅ | Windows focus, taskbar, seed | Win32 focus capture/restore; `skipTaskbar`; `hello.ps1` seed; `.ps1` invocation via PowerShell |
| 26 ✅ | Cross-platform CI & packaging | GitHub Actions matrix (macOS/Linux/Windows); `.dmg`, `.flatpak`, `.msi` artefacts; per-platform build docs |
| 27 ✅ | Co-located resources | **Breaking change:** static lists and scripts moved from `lists/` and `scripts/` to sit next to their command YAML |
| 28 ✅ | Built-in script environment variables | Inject `NIMBLE_*` variables into every script execution path |
| 29 ✅ | User-defined script variables | Add global and command-scoped user variables with deterministic precedence |
| 30 | Script debugging and verbose logs | Add debug mode with `NIMBLE_DEBUG` and structured script diagnostics |
| 31 ✅ | External script/list paths | `${VAR}` substitution in `script:` / `list:` fields; `allow_external_paths` setting |
| 32 ✅ | Copilot agents | `@nimble-command` and `@nimble-script` GitHub Copilot agents for command authoring |
| 33 ✅ | Branding & app identity | D3 Warm Neon icon; `tauri icon` asset generation; bundle identifier `io.switchpanel.nimble` |
| 34 ✅ | Agent spec refactor | Canonical `nimble-spec.yaml`; thin agent pointers; rule 12a |
| 35 ✅ | Static list TSV format | **Breaking change:** list files switched from YAML to TSV for human editability |
| 36 ✅ | Docs restructure | Replaced `using/basic/` + `using/advanced/` with `actions/`, `guides/`, `reference/` |
| 37 ✅ | UI polish & window dragging | Draggable window; backdrop blur; layered shadows; prompt glyph; accent selection indicator; action-type badges |
| 38 ✅ | Spec & agent versioning | Independent integer `spec_version` for spec and agents; broadened bump rules; published to public repo |
| 39 ✅ | `/nimble docs` built-in command | Five doc topics open in browser; deploying-agents guide; `docs_open` builtin action type |

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
  - **macOS**: `~/Library/Application Support/ContextActions/`
  - **Linux**: `$XDG_CONFIG_HOME/ContextActions/` (falls back to `~/.config/ContextActions/`)
  - **Windows**: `%APPDATA%\ContextActions\`
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

## Stage 10 — App Rename: Contexts → Context Actions ✅

**Goal:** Rename the application from *Contexts* to *Context Actions* everywhere — product name, bundle identifier, config directory, localStorage keys, log prefixes, and all documentation. This is a breaking change to the config directory path; existing users must migrate their command files.

### Changes

| Location | Before | After |
|----------|--------|-------|
| Product name (`tauri.conf.json`) | `Contexts` | `Context Actions` |
| Bundle identifier | `com.contexts.launcher` | `ContextActions` |
| Cargo package name | `contexts-launcher` | `context-actions` |
| Cargo lib name | `contexts_launcher_lib` | `ctx_launcher_lib` |
| npm package name | `contexts-launcher` | `context-actions` |
| Config dir (macOS) | `~/Library/Application Support/com.contexts.launcher/` | `~/Library/Application Support/ContextActions/` |
| localStorage hotkey key | `contexts_hotkey` | `ctx_hotkey` |
| Log prefix | `[contexts]` | `[ctx]` |
| Onboarding title | `Welcome to Contexts` | `Welcome to Context Actions` |
| Tray menu items | `Contexts vX.Y.Z` / `Quit Contexts` | `Context Actions vX.Y.Z` / `Quit Context Actions` |

### Migration note
Because the bundle identifier changes, Tauri will use a new config directory. Users upgrading from the *Contexts* build must manually move their command files:
```bash
mv ~/Library/Application\ Support/com.contexts.launcher \
   ~/Library/Application\ Support/ContextActions
```

### Done when ✅
- App builds and runs under the new name and identifier
- Config directory is `ContextActions`; command files load correctly
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

## Stage 13 — Backend Testing ✅

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

## Stage 14 — Action: Show List ✅

**Goal:** Commands can reference a named list stored in the `lists/` config subdirectory. As soon as the user's typed input exactly matches the command phrase, the full list of items is shown immediately — no Enter key required. Selecting an item pastes its value into the previously focused application.

### List file format

List files live in `config_dir/lists/` and use a plain YAML array. Each entry has a required `title` and an optional `subtext`. Blank lines between items and `#` comments are supported to aid readability.

**Example** — `lists/team-emails.yaml`:
```yaml
# Team email addresses
- title: Alice Smith
  subtext: alice@example.com

- title: Bob Jones
  subtext: bob@example.com

- title: Carol White
  subtext: carol@example.com
```

| Field | Required | Description |
|-------|----------|-------------|
| `title` | ✅ | Display label shown as the result title |
| `subtext` | No | Secondary display line; also the value pasted on selection. Falls back to `title` if absent. |

### Command schema

A command references a list by filename (without extension). The file is resolved relative to `config_dir/lists/`.

```yaml
phrase: team emails
title: Team email addresses
action:
  type: static_list
  config:
    list: team-emails    # resolves to lists/team-emails.yaml
```

### Behaviour
- **Partial match:** typing part of the phrase (e.g. `"team"`) shows the command as a single result row, identical to any other command type.
- **Exact match trigger:** the moment the typed text equals the phrase exactly (e.g. `"team emails"`), the single result row is replaced by the full list of items — no Enter required.
- **Return to normal:** if the user edits the input so it no longer exactly matches, the list collapses and standard partial-match results are shown again.
- **Selection:** pressing Enter or clicking an item runs `paste_text` with the item's `subtext` (or `title` if `subtext` is absent), then dismisses the launcher.
- **Window resize:** the window grows to accommodate the list, capped at 8 visible items (same resize logic as the existing results list).

### Tasks

#### Rust backend (`commands.rs`)
- Add `StaticListConfig { list: String }` struct and `StaticList(StaticListConfig)` variant to the `Action` enum
- Add `ListItem { title: String, subtext: Option<String> }` struct (serialisable + deserialisable)
- Add `pub(crate) fn load_list(config_dir: &Path, list_name: &str) -> Result<Vec<ListItem>, String>`:
  - Reject `list_name` values containing `/`, `\`, or `..` components (path traversal prevention)
  - Read and parse `config_dir/lists/<list_name>.yaml`
  - Return `Err` with a clear message if the file is missing or malformed
- Expose `load_list` as a Tauri command so the frontend can invoke it
- Extend the file watcher in `watcher.rs` to also watch `config_dir/lists/` and emit the same `commands://reloaded` event on changes

#### Frontend (`types.ts`, `+page.svelte`)
- Add `StaticListConfig`, `static_list` action variant, and `ListItem` interface to `types.ts`
- In `+page.svelte`, add reactive logic to detect an exact phrase match for a `static_list` command:
  - Invoke `load_list` with the list name; store result in a `listItems` state variable
  - Render the `listItems` list instead of the standard results list
  - On `commands://reloaded`, re-invoke `load_list` if a list is currently displayed
- Selecting a list item invokes `paste_text` with `item.subtext ?? item.title`; clears input and dismisses

#### Config & docs
- Add seed files: `lists/team-emails.yaml` and `commands/examples/show-team-emails.yaml`
- Update `docs/using/config-directory.md` to document the `lists/` subdirectory
- Update `docs/using/basic-functionality.md` with a Show List section
- Update `copilot-instructions.md` rule 5 to include `static_list` in the list of action types

#### Backend tests (`commands.rs`)
- `load_list` with a valid YAML file returns the correct `Vec<ListItem>`
- Items containing only `title` (no `subtext`) deserialise correctly
- A non-existent list name returns `Err`
- A list name containing `..` or `/` is rejected with `Err`

### Done when
- Typing a `static_list` phrase exactly causes the list items to appear without pressing Enter
- Partial typing still shows the command as a single result row
- Selecting a list item pastes its subtext (or title if absent) into the previously active application
- Editing a list file hot-reloads the displayed items within the debounce window

---

## Stage 15 — Action: Dynamic List ✅

**Goal:** Commands can execute an external script to dynamically generate a list of items. Like `static_list`, the list appears automatically when the phrase is matched — no Enter required. Scripts can optionally accept a user-typed argument to filter or parameterise their output in real time.

### Script output format

Scripts live in `config_dir/scripts/` and can be any executable (shell script, Python, Node.js binary, etc.). Each script must write to stdout either:

- **Plain text** — treated as a single result with the output as the item title
- **JSON array** — a list of items, each conforming to `{ "title": "string", "subtext": "string" }` (subtext is optional)

Stdout is captured and parsed. Stderr is discarded. A 5-second timeout applies; if the script does not complete in time, an empty list is shown.

### Command schema

```yaml
phrase: team emails
title: Team email addresses
action:
  type: dynamic_list
  config:
    script: team-emails.sh    # resolves to scripts/team-emails.sh
    arg: none                 # none | optional | required
    item_action: paste_text   # optional; same semantics as static_list
```

### Argument modes

| `arg` value | Trigger condition | Script invoked with |
|-------------|-------------------|---------------------|
| `none` | Input exactly equals phrase | No arguments |
| `optional` | Input exactly equals phrase, **or** input starts with phrase + space | No argument (exact match) or the typed suffix (with suffix) |
| `required` | Input starts with phrase + space and suffix is non-empty | The typed suffix as first argument |

- **`none`:** Script runs once on exact match; typing extra text beyond the phrase collapses the list (identical behaviour to `static_list`).
- **`optional`:** List appears immediately on exact match (no argument passed); as the user continues typing a suffix the script is re-invoked with that suffix — results update in real time. A 200 ms debounce prevents excessive invocations.
- **`required`:** No results shown on exact match alone. The list appears only once the user has typed at least one character after the phrase (the suffix is passed as the script argument).

### `item_action`

Works identically to `static_list`: if omitted, selecting an item dismisses the launcher. Optional values: `paste_text`, `copy_text`, `open_url`.

### Security boundaries (non-negotiable)
- `script` field values containing `/`, `\`, or `..` are rejected at invocation time (path traversal prevention)
- Scripts are resolved relative to `config_dir/scripts/` only and run with the same privileges as the launcher — never elevated
- Script output is validated before any action is taken; malformed JSON is silently discarded (empty list shown)
- Only `paste_text`, `copy_text`, and `open_url` (http/https only) are valid `item_action` values

### Tasks

#### Rust backend (`commands.rs`, `lib.rs`)
- Add `ArgMode` enum: `None`, `Optional`, `Required` (serde: `snake_case`)
- Add `DynamicListConfig { script: String, arg: ArgMode, item_action: Option<ItemAction> }` struct and `DynamicList(DynamicListConfig)` variant to the `Action` enum
- Add `pub fn run_script(config_dir: &Path, script_name: &str, arg: Option<&str>) -> Result<Vec<ListItem>, String>`:
  - Reject `script_name` values containing `/`, `\`, or `..`
  - Resolve to `config_dir/scripts/<script_name>`; return `Err` if the file does not exist
  - Spawn subprocess; pass `arg` as a positional argument if `Some`; capture stdout; enforce 5 s timeout
  - Parse stdout: try JSON array (`Vec<ListItem>`) first; fall back to a single `ListItem { title: trimmed_output, subtext: None }`
  - Return `Err` with a clear message on timeout or malformed JSON
- Expose as `run_dynamic_list` Tauri command in `lib.rs`
- Extend `watcher.rs` to watch `config_dir/scripts/` (creating it eagerly if missing) alongside `commands/` and `lists/`

#### Frontend (`types.ts`, `+page.svelte`)
- Add `ArgMode`, `DynamicListConfig`, and `dynamic_list` action union member to `types.ts`
- In `+page.svelte`, extend the exact-match `$effect` to handle `dynamic_list`:
  - `none` / `optional` (exact match): invoke `run_dynamic_list` with no argument; populate `listItems`
  - `optional` / `required` (phrase + space + suffix): invoke `run_dynamic_list` with the suffix; debounce 200 ms
  - `required` (exact match only): do nothing — leave `listItems` empty so no list is shown
- Reuse the existing `listItems`, `activeListCmd`, and `showingList` state from `static_list`
- On `commands://reloaded`, re-invoke `run_dynamic_list` with the current argument if a dynamic list is displayed
- Item selection uses the same `executeListItem` path as `static_list`

#### Config & docs
- Watcher creates `config_dir/scripts/` on startup if it does not exist
- Add a seed example: `scripts/hello.sh` (echoes a JSON array) and `commands/examples/dynamic-list-example.yaml`
- Update `docs/using/config-directory.md` to document the `scripts/` subdirectory
- Update `docs/using/basic-functionality.md` with a Dynamic List section
- Update `copilot-instructions.md` rule 5 to include `dynamic_list`

#### Backend tests (`commands.rs`)
- `run_script` with a valid JSON-outputting script returns the correct `Vec<ListItem>`
- `run_script` with plain-text output wraps it in a single `ListItem`
- A script name containing `..` or `/` is rejected with `Err`
- A non-existent script name returns `Err`
- `DynamicListConfig` with all three `arg` modes deserialises correctly

### Done when
- `arg: none` — exact phrase match executes the script and shows results immediately; extra typing collapses the list
- `arg: optional` — results appear on exact match; typing a suffix re-runs the script live with that suffix
- `arg: required` — no results shown until the user types a non-empty suffix after the phrase
- Selecting a list item executes the configured `item_action` (or dismisses if absent)
- Scripts in `config_dir/scripts/` are watched; editing a script hot-reloads any currently displayed results

---

## Stage 16 — Docs Restructure & Cleanup ✅

**Goal:** Reorganise user-facing documentation for clarity and long-term maintainability. Separate basic from advanced functionality, add proper landing pages, fill onboarding gaps, and remove stale content.

### Changes made

#### Structure
- Split the single `basic-functionality.md` into per-action pages under `docs/using/basic/` (`open-url.md`, `paste-text.md`, `copy-text.md`) with a `README.md` landing page
- Created `docs/using/advanced/` with per-action pages (`static-list.md`, `dynamic-list.md`) and moved `script-extensions.md` there, with a `README.md` landing page
- Created `docs/using/README.md` as an orientating landing page for the whole `using/` folder

#### README
- Added a "How it works" section explaining the YAML → command → match mental model
- Added a "Your first command" section with a minimal copy-paste `open_url` example
- Renamed "Getting Started" → "Building from source" to distinguish end-user and developer paths
- Removed stale `plugins/` entry from the Project Structure block
- Updated the Using Context Actions table to reference the new `basic/README.md` and `advanced/README.md`

#### First-run guide
- Added a "Your first command" section: config directory path, a worked YAML example, and what to expect when typing the phrase

#### Content fixes
- `tips-and-tricks.md`: fixed stale opening sentence that said "two built-in actions"
- `advanced/dynamic-list.md`: clarified dual role of `subtext` (display hint *and* action payload); added cross-link to `script-extensions.md` for argument details
- `advanced/static-list.md`: added matching `subtext` note
- `docs/using/config-directory.md`: updated links to the new folder structure

#### Copilot instructions
- Rule 1: added `git push` requirement after every commit
- Rule 6: replaced generic "keep docs up to date" with an explicit classify-before-writing rule — ask whether a new feature is basic or advanced before placing it
- Updated built-in actions list and file/folder conventions to reflect current state

#### Seed examples in config directory
- Added `scripts/hello.sh` (JSON-outputting example script)
- Added `commands/examples/dynamic-list-example.yaml`

### Done when ✅
- All action types have a dedicated page in the correct `basic/` or `advanced/` subfolder
- A newcomer can understand the app, set up a first command, and reach advanced features through a clear link hierarchy
- No stale references to old file paths remain in any doc

---

## Stage 17 — Action: Script Action ✅

**Goal:** A new action type that executes an external script on `Enter` (like `open_url` and `paste_text`) rather than auto-triggering an inline list. The script's output — a single value or a list of values — is acted upon immediately using a configured basic action.

This is distinct from `dynamic_list`, which displays an interactive list for the user to select from. `script_action` runs to completion and applies the result directly: it opens URLs, pastes text, or copies to clipboard without further user interaction.

### Script output format

Scripts live in `config_dir/scripts/` (same directory as `dynamic_list` scripts). They must write to stdout either:

- **Plain text** — the entire trimmed stdout is used as a single value
- **JSON array of strings** — `["value1", "value2", ...]` — treated as a list of values

A 5-second timeout applies; if the script does not exit in time, execution is aborted.

### Command schema

```yaml
phrase: get ticket url
title: Open current Jira ticket
action:
  type: script_action
  config:
    script: get-ticket.sh  # resolves to scripts/get-ticket.sh
    arg: none              # none (default) | optional | required
    result_action: open_url  # open_url | paste_text | copy_text
```

With prefix/suffix for list paste/copy:

```yaml
phrase: paste emails
title: Paste all team emails
action:
  type: script_action
  config:
    script: team-emails.sh
    result_action: paste_text
    prefix: ""
    suffix: "\n"
```

### Argument modes

| `arg` value | When execution is allowed | Script receives |
|-------------|---------------------------|-----------------|
| `none` *(default)* | Any time the command is selected | No arguments |
| `optional` | Any time the command is selected | Suffix after phrase, if present |
| `required` | Only when a non-empty suffix follows the phrase | The typed suffix as its first argument |

The suffix is the text the user types after the command phrase, extracted the same way as `{param}` in `open_url`.

### Result actions

#### `open_url`
Each output value is opened as a URL in the default browser. Values are opened individually. Standard URL scheme validation applies (http/https; deep-link schemes permitted).

#### `paste_text`
All output values are concatenated after wrapping each with the optional `prefix` and `suffix` fields. The combined string is pasted into the previously focused application in a single operation. If `prefix` and `suffix` are omitted, values are concatenated without decoration.

#### `copy_text`
Same as `paste_text` but copies to the clipboard without simulating a keystroke.

**Examples:**

With `prefix: "" suffix: "\n"` and output `["alice@example.com", "bob@example.com"]`:
```
alice@example.com
bob@example.com
```

With `prefix: "<" suffix: ">"` and output `["alice@example.com", "bob@example.com"]`:
```
<alice@example.com><bob@example.com>
```

### Security boundaries
- `script` values containing `/`, `\`, or `..` are rejected at invocation time
- For `open_url` result action, each URL is passed through the existing scheme validation
- For `paste_text` / `copy_text`, NUL-byte validation applies to the assembled string

### Tasks

#### Rust backend (`commands.rs`, `lib.rs`)
- Add `ResultAction` enum: `OpenUrl`, `PasteText`, `CopyText` (serde: `snake_case`)
- Add `ScriptActionConfig { script: String, arg: ArgMode, result_action: ResultAction, prefix: Option<String>, suffix: Option<String> }` and `ScriptAction(ScriptActionConfig)` variant to the `Action` enum
- Add `pub fn run_script_values(config_dir: &Path, script_name: &str, arg: Option<&str>) -> Result<Vec<String>, String>`:
  - Reject names containing `/`, `\`, or `..`
  - Spawn script with optional arg; 5 s timeout
  - Parse stdout: try `Vec<String>` JSON first; fall back to `vec![stdout.trim().to_string()]`
- Expose as `run_script_action` Tauri command in `lib.rs`

#### Frontend (`types.ts`, `+page.svelte`)
- Add `ResultAction`, `ScriptActionConfig`, and `script_action` union member to `types.ts`
- In `executeCommand()`, handle `script_action`:
  - Extract suffix from input (text after phrase)
  - Guard: if `arg: required` and no suffix, do nothing
  - Invoke `run_script_action`; receive `Vec<String>`
  - `open_url`: invoke `open_url` for each value individually
  - `paste_text` / `copy_text`: assemble `values.map(v => prefix + v + suffix).join('')`; invoke the action once

#### Docs
- Add `docs/using/advanced/script-action.md` with full schema, examples, and result action behaviour
- Update `docs/using/advanced/README.md` to link to the new page
- Update `docs/using/configuring-commands.md` full schema to include `script_action`

#### Seed example
- Add `commands/examples/script-action-example.yaml`

#### Backend tests (`commands.rs`)
- `run_script_values` with JSON array output returns correct `Vec<String>`
- `run_script_values` with plain text output returns single-element vec
- Script name with path traversal is rejected
- Non-existent script returns `Err`
- `ScriptActionConfig` with all `arg` modes and `result_action` variants deserialises correctly

### Done when
- Selecting a `script_action` command runs the script and immediately applies the result (no intermediate list shown)
- `open_url`: each returned URL opens in the browser
- `paste_text` / `copy_text`: returned values are assembled with prefix/suffix and pasted/copied in one operation
- `arg: required` commands do not execute without a suffix
- Scripts with invalid names are rejected

---

## Stage 18 — Example Config Directory ✅

**Goal:** Add an `example-config/` directory to the repository containing a complete, copy-pasteable reference config that exercises every action type. Users can copy it into their own config directory to get running immediately.

### Structure

```
example-config/
├── commands/
│   └── examples/
│       ├── open-google.yaml           # open_url — simple URL
│       ├── open-github.yaml           # open_url — simple URL
│       ├── open-reddit.yaml           # open_url — simple URL
│       ├── open-slack.yaml            # open_url — deep link (Slack)
│       ├── open-notes.yaml            # open_url — deep link (Obsidian)
│       ├── open-morning-sites.yaml    # script_action / open_url — opens list of URLs
│       ├── search-google.yaml         # open_url — {param} substitution
│       ├── paste-email.yaml           # paste_text
│       ├── paste-greeting.yaml        # paste_text — multi-line template
│       ├── paste-team-emails.yaml     # script_action / paste_text — list with suffix
│       ├── paste-team-emails-as-task.yaml  # script_action / paste_text — prefix + suffix
│       ├── copy-email.yaml            # copy_text
│       ├── copy-uuid.yaml             # script_action / copy_text
│       ├── show-team-emails.yaml      # static_list
│       ├── dynamic-list-example.yaml  # dynamic_list — filterable list
│       └── script-action-example.yaml # script_action / paste_text — timestamp
├── lists/
│   └── team-emails.yaml
└── scripts/
    ├── hello.sh           # dynamic_list script — filterable greeting list
    ├── timestamp.sh       # script_action script — current date/time
    ├── uuid.sh            # script_action script — random UUID
    ├── team-emails.sh     # script_action script — list of email strings
    └── morning-sites.sh   # script_action script — list of URLs
```

### Done when
- `example-config/` exists in the repository root with all files above
- Every action type (`open_url`, `paste_text`, `copy_text`, `static_list`, `dynamic_list`, `script_action`) is represented by at least one command
- `example-config/README.md` describes the structure and lists all commands

---

## Stage 19a — Contexts: Reserved Namespace (Backend) ✅

**Goal:** Enforce the `ctx` reserved namespace in the Rust backend. Any user-defined YAML command whose phrase starts with `ctx` is rejected at load time and surfaces a warning to the frontend, preventing namespace collisions with the built-in context commands that follow in Stage 19b.

This is a pure backend change. No frontend modifications are needed yet.

### Tasks

#### `commands.rs` — new warning variant
- Add a second variant to the warning type alongside `DuplicateWarning`:
  ```rust
  pub struct ReservedPhraseWarning {
      pub phrase: String,   // the rejected phrase (as written in the YAML file)
      pub file: String,     // config-dir-relative path of the offending file
  }
  ```
- Wrap both warning kinds in a `Warning` enum (or keep separate `Vec` fields in `LoadResult` — either is fine, choose whichever keeps the frontend change minimal):
  ```rust
  pub struct LoadResult {
      pub commands: Vec<Command>,
      pub duplicates: Vec<DuplicateWarning>,
      pub reserved: Vec<ReservedPhraseWarning>,   // new
  }
  ```

#### `commands.rs` — rejection logic
- In `load_from_dir`, after successfully parsing a `Command`, check whether its phrase starts with `ctx` (case-insensitive), optionally followed by a space or end of string.
- If so, skip the command and push a `ReservedPhraseWarning` instead of adding it to `commands`.
- The check must be case-insensitive: `"Context Actions"`, `"CTX"`, `"ctx"` are all rejected.

#### `types.ts` — new warning interface
- Add `ReservedPhraseWarning { phrase: string; file: string }` interface.
- Extend `CommandsPayload` with `reserved: ReservedPhraseWarning[]`.

#### `+page.svelte` — surface in warnings bar
- Combine `reserved` warnings with `duplicates` for the existing warnings count in the warnings bar:
  `warnings.length` used in the bar should reflect `duplicates.length + reserved.length`.
- No separate UI treatment required at this stage — they can share the same dismissable banner.

#### Backend tests (`commands.rs`)
- A command with phrase `"ctx set foo"` is rejected and appears in `reserved`, not `commands`
- A command with phrase `"CTX reset"` (uppercase) is also rejected
- A command with phrase `"ctxfoo"` (no space — not a `ctx` prefix) is accepted normally
- A command with phrase `"open ctx"` (contains but doesn't start with `ctx`) is accepted normally
- `reserved` vec is empty when no phrase violations exist

### Done when
- `cargo test` passes with the new tests
- A user YAML file with `phrase: ctx anything` is silently ignored by the loader and a `ReservedPhraseWarning` is included in the load result
- The frontend warnings bar count includes reserved-phrase violations

---

## Stage 19b — Contexts: State & Built-in Commands (Frontend) ✅

**Goal:** Introduce `activeContext` state and the built-in `ctx` commands that manage it. After this stage the user can type `ctx set reddit`, press Enter, and see the context update — but matching still uses raw input (context-aware matching comes in Stage 19c).

### Concepts (shared across 19a–19c)

A **context** is a non-empty string stored in `activeContext` frontend state. The key invariant: when context is active and the user's raw input does **not** start with `ctx`, matching uses `effective_input = raw_input + " " + context`. When context is empty (default), `effective_input = raw_input` — zero behaviour change from before.

**Built-in commands** are generated synthetically in the frontend, not loaded from YAML. They use the reserved `ctx` prefix and are always present regardless of what YAML files the user has.

### Tasks

#### `+page.svelte` — `activeContext` state
- Add `let activeContext = $state("")` — empty string means no context.
- Built-in commands are a `$derived` value computed from `activeContext`:
  ```ts
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
  ```

#### `types.ts` — `builtin` action type
- Add a `BuiltinConfig { action: "ctx_set" | "ctx_reset" | "ctx_show" }` interface.
- Add `| { type: "builtin"; config: BuiltinConfig }` to the `Action` union.
- The `builtin` type is frontend-only and never appears in YAML files.

#### `+page.svelte` — filtering built-in commands
- When the user's raw input starts with `ctx` (case-insensitive), include `builtinCommands` in the results alongside (or instead of) regular YAML commands that match.
- Built-in commands use the same partial-match logic (`phrase.includes(typed)`).
- `ctx set` is a special case: it matches when the user types `"ctx set"` exactly, or `"ctx set <anything>"` — the text after `"ctx set "` is the new context value.

#### `+page.svelte` — `executeCommand` for builtins
- Add a `builtin` branch in `executeCommand`:
  - `ctx_set`: extract the suffix after `"ctx set "` from the raw input; if non-empty, set `activeContext = suffix.trim()`; clear input; **do not** dismiss the launcher.
  - `ctx_reset`: set `activeContext = ""`; clear input; **do not** dismiss the launcher.
  - `ctx_show`: no-op — just clear input; do not dismiss.

#### Subtext display for `ctx set`
- In the results list, when the selected row is the `ctx set` command and the user has typed `"ctx set foo"`, show subtext `→ set context to "foo"` so the user sees a preview of what will happen.

### Done when
- Typing `ctx` shows the three built-in commands in the results list
- Typing `ctx set reddit` and pressing Enter sets `activeContext` to `"reddit"`; launcher stays open
- Typing `ctx reset` and pressing Enter clears `activeContext`; launcher stays open
- `ctx show` displays the current context in its title and does nothing on Enter
- All other (non-`ctx`) inputs behave exactly as before this stage

---

## Stage 19c — Contexts: Context-Aware Matching ✅

**Goal:** Wire `activeContext` into the filtering and execution pipeline so that the context suffix is transparently appended to the user's raw input before matching. After this stage the full context feature is usable end-to-end.

### Rules (recap from plan)

```
effective_input = raw_input + " " + activeContext   (context non-empty AND raw_input does not start with "ctx")
effective_input = raw_input                         (context empty OR raw_input starts with "ctx")
```

### Tasks

#### `+page.svelte` — `effectiveInput` derived value
- Add a `$derived` value:
  ```ts
  const effectiveInput = $derived(
    activeContext && !input.trim().toLowerCase().startsWith("ctx")
      ? input.trim() + " " + activeContext
      : input.trim()
  );
  ```

#### Filtering
- Replace all uses of `input.trim()` (or its lowercased form) in the `filtered` derived value with `effectiveInput`.
- `builtinCommands` are filtered using **raw input** (`input.trim()`) only — they bypass `effectiveInput`.

#### Exact-match list triggers
- The `$effect` that detects `static_list` / `dynamic_list` exact-phrase matches must compare against `effectiveInput.toLowerCase()` rather than `input.trim().toLowerCase()`.

#### `executeCommand` — param extraction
- In the `open_url` branch, the suffix extracted as the `{param}` value must be derived from `effectiveInput` (not raw input), so context-as-parameter works correctly.
- In the `script_action` branch, same: extract the suffix from `effectiveInput`.

#### Highlight subtext
- When a context is active the subtext highlight logic should use `effectiveInput` for matching so the highlighted portion is accurate.

#### Backend tests (none new — pure frontend logic)
- No new Rust tests needed; the matching is entirely in the frontend.

### Done when
- With `activeContext = "reddit"`, typing `"open"` matches a command with phrase `"open reddit"`
- With `activeContext = "rust programming"`, typing `"search google"` executes with param `"rust programming"`
- With `activeContext = ""` (default), all commands behave exactly as before Stage 19a
- Typing `"ctx set foo"` still works regardless of what `activeContext` is (raw input bypasses context appending)

---

## Stage 20 — Contexts: UI Indicators & Tray Integration ✅

**Goal:** Make the active context visible at all times — both inside the launcher window and in the system tray — so the user always knows which context is in effect.

### Tasks

#### Launcher input area
- When a context is active, display a non-editable **context chip** to the right of the text input inside the launcher bar (e.g. a small pill badge showing `reddit ×`). Positioning on the right reflects that the context is a suffix appended to whatever the user types. The chip is styled distinctly (e.g. blue background, rounded).
- The `×` on the chip clears the context immediately (same as `ctx reset`).
- The input placeholder changes to `"Type a command…"` when no context is set, and `"…"` (or similar) when a context is active, to leave visual room for the chip.
- The chip is purely cosmetic / a shortcut — it does not affect the input field's value.

#### Window height
- Account for the chip row height in the dynamic window-resize `$effect` when a context is active and the chip is shown.

#### System tray
- ~~When a context is active, append the context name to the tray tooltip / app-info menu item, e.g. `"Context Actions — reddit"`.~~
- ~~When no context is active, show the default `"Context Actions vX.Y.Z"` label.~~
- The tray is not updated when the context changes; it always shows `"Context Actions vX.Y.Z"`.

#### Persistence
- Active context is stored in `localStorage` under the key `ctx_active_context` so it survives launcher restarts.
- On mount, restore the saved context (if any) before the first filter pass.

### Done when
- The active context is shown as a chip in the launcher bar with a clear button
- The system tray info item reflects the active context name
- The context persists across launcher restarts (hide/show cycles and full quit-relaunch)

---

## Stage 21 — Settings File ✅

**Goal:** Persist application settings to a `settings.yaml` file in the app config directory, replacing the old `localStorage` hotkey storage. Gives users a human-editable config for the global hotkey, the context chip visibility, and deduplication behaviour.

### Tasks

- Add `settings.rs` with `AppSettings { hotkey, show_context_chip, allow_duplicates }`, `load()`, and `save()` helpers.
- `allow_duplicates` defaults to `true` (no dedup). When `false`, first-file-wins deduplication is applied and `DuplicateWarning`s are emitted.
- Update `commands::load_from_dir` to accept `allow_duplicates: bool`.
- Update `watcher::start` to accept and forward `allow_duplicates`.
- Add `SettingsState` managed state in `lib.rs`; load settings in `setup()` and register the hotkey immediately if present.
- Add Tauri commands `get_settings` and `save_hotkey`.
- `list_commands` reads `allow_duplicates` from `SettingsState`.
- Frontend: `onMount` calls `get_settings` instead of reading `localStorage`; one-time migration from old localStorage keys to `settings.yaml`.
- `confirmShortcut` calls `save_hotkey` instead of `localStorage.setItem`.
- `show_context_chip` gates the context chip and placeholder text.
- Add `example-config/settings.yaml` with commented documentation.
- Install live example to `~/Library/Application Support/ContextActions/settings.yaml`.

### Done when ✅
- The global hotkey is stored in `settings.yaml`, not `localStorage`
- Existing users retain their hotkey after upgrade (one-time migration on first launch)
- `allow_duplicates: false` enables dedup warnings; changing it takes effect on next relaunch
- `show_context_chip: false` hides the chip and restores the normal placeholder

---

## Stage 22 — Housekeeping ✅

**Goal:** Tidy up the project before further feature work — consistent naming, a licence, orientation documents for new contributors, and a well-organised docs tree.

### Tasks

#### App rename
- Renamed product from `Ctx` / `Ctx Launcher` to **Context Actions** across all surfaces: `tauri.conf.json` (`productName`, `identifier`), `Cargo.toml` (package and lib name), `package.json`, `app.html`, `lib.rs` tray strings, onboarding welcome text, and all documentation.
- New bundle identifier: `ContextActions`; new config directory: `~/Library/Application Support/ContextActions/`.
- `ctx set / ctx reset / ctx show` command phrases and internal `ctx_` identifiers left unchanged (contexts feature namespace, not app brand).

#### MIT licence
- Added `LICENSE` file (MIT, 2026, Harpreet Singh Gulati).
- Added `license = "MIT"` field to `Cargo.toml`; `package.json` already declared MIT.
- Updated README licence section from "TBD" to "MIT — see LICENSE".

#### Roadmap
- Added `docs/roadmap.md` listing all implemented features (checked) and planned items grouped by theme: Distribution & Updates, Theming, Configuration & Usability, Actions, Community.

#### Motivation doc
- Added `docs/motivation.md` explaining why Context Actions exists, its goals (reduce friction, context-switching overhead, distraction), why not Alfred/Raycast, and explicit non-goals (not an app launcher, not a full replacement, no plugin system).

#### Docs cleanup
- Renamed `docs/using/advanced/script-extensions.md` → `writing-scripts.md` (better reflects content as a how-to guide).
- Fixed stale name references (`Contexts Launcher`, `contexts-launcher`) in `copilot-instructions.md`, `development-setup.md`, and `README.md`.
- Removed Contributing section from README (roadmap item added to re-add it later).
- Verified all internal doc links — zero broken links.
- Updated `advanced/README.md` and `docs/using/README.md` to include Contexts in the Advanced features description.

#### Copilot instructions
- Added rule 11: update `docs/roadmap.md` when a planned item is implemented, marking its checkbox and updating the description if needed.

### Done when ✅
- App is consistently named "Context Actions" across all files
- MIT licence file is present and referenced
- Roadmap and motivation docs are committed
- Docs tree has no broken links and no stale name references

---

## Stage 23 — Cross-Platform Clipboard (Linux & Windows)

### Goal
Replace the macOS-only `pbcopy` clipboard implementation with the `arboard` crate so that `copy_text` and `paste_text` work on Linux and Windows without per-action code changes.

### Background
`write_clipboard_text()` currently forks a `pbcopy` subprocess on macOS and returns `Err("paste_text is not yet supported on this platform")` on all other platforms. `arboard` is a pure-Rust, cross-platform clipboard crate that supports macOS, Linux (X11 + Wayland), and Windows. The `notify` crate's `macos_fsevent` feature is also incorrectly declared as an unconditional dependency — it should be scoped to macOS.

### Tasks
- Add `arboard = "3"` to `[dependencies]` in `Cargo.toml`
- Move `notify` to `[dependencies]` without the `macos_fsevent` feature; add a `[target.'cfg(target_os = "macos")'.dependencies]` entry for `notify` with `features = ["macos_fsevent"]`
- Rewrite `write_clipboard_text()`:
  - macOS: keep `pbcopy` subprocess (avoids NSPasteboard threading constraints) **or** switch to `arboard` if testing confirms equal reliability
  - Linux & Windows: use `arboard::Clipboard::new()?.set_text(text)`
- Clipboard integration tests require a display server — gate them with `#[ignore]` and document that `DISPLAY` must be set for them to run
- Verify `paste_text` (clipboard write + `enigo` Ctrl+V) works end-to-end on Linux in a VM or container with a display

### Done when ✅
- `copy_text` and `paste_text` compile and run on Linux and Windows without returning "not supported"
- `cargo test` passes on all three platforms; clipboard integration tests marked `#[ignore]` for headless CI
- `notify` no longer requests `macos_fsevent` on non-macOS builds

---

## Stage 24 — Linux: Focus Tracking & Paste Flow

### Goal
Implement `capture_previous_app` and `restore_previous_app` on Linux (X11) so that `paste_text` correctly returns focus to the user's previous application before simulating Ctrl+V.

### Background
On Linux the two functions are no-ops; focus is never restored before the `enigo` keystroke fires, so the paste lands in the wrong window or is silently dropped. Two approaches:
1. **`xdotool` subprocess** — `xdotool getactivewindow` to capture, `xdotool windowfocus --sync <id>` to restore. Widely available on X11 desktops; zero Rust crate deps.
2. **`xcb`/`x11rb`** — query `_NET_ACTIVE_WINDOW` directly in Rust. More robust, but adds a heavy dependency.

Chosen approach: `xdotool` subprocess (simpler, no new crate, aligns with how `pbcopy` works on macOS).

### Tasks
- `capture_previous_app` on Linux: run `xdotool getactivewindow`; store the window ID as a string in `PreviousApp` state
- `restore_previous_app` on Linux: run `xdotool windowfocus --sync <id>` + `xdotool windowraise <id>` as a subprocess; ignore errors gracefully
- Wayland: detect `WAYLAND_DISPLAY` set and `DISPLAY` unset; skip focus restoration and return `Ok(())`; log a warning so the user can understand why paste doesn't focus
- Update `PreviousApp` state to store `String` (PID string on macOS, window-ID string on Linux) to avoid per-platform type divergence
- Add `xdotool` to the Linux system-dependency list in `docs/development-setup.md` and `docs/using/config-directory.md`

### Done when ✅
- On Linux X11, `paste_text` focuses the previous app and pastes correctly
- On Wayland, `paste_text` writes to clipboard without crashing; a warning is logged about focus limitation
- `cargo test` still passes on all platforms

---

## Stage 25 — Windows: Focus Tracking, Taskbar & Seed Script

### Goal
Make the launcher fully functional on Windows: pre-invoke focus capture, post-dismiss focus restore, hiding from the Windows taskbar, and a seed script the user can actually execute.

### Tasks

#### Focus tracking
- `capture_previous_app` on Windows: call `GetForegroundWindow()` via `windows-sys` crate; store the `HWND` value as a `u64` in `PreviousApp` state
- `restore_previous_app` on Windows: call `SetForegroundWindow(hwnd)` then `BringWindowToTop(hwnd)`
- Add `[target.'cfg(target_os = "windows")'.dependencies]` block: `windows-sys = { version = "0.59", features = ["Win32_UI_WindowsAndMessaging"] }`

#### Taskbar hiding
- Add `"skipTaskbar": true` to the window entry in `tauri.conf.json`; Tauri 2 maps this to `WS_EX_TOOLWINDOW` on Windows so the window is excluded from the taskbar

#### Seed script
- In `watcher.rs`, conditionally seed `hello.sh` on macOS/Linux and `hello.ps1` on Windows
- `hello.ps1`: PowerShell equivalent that `Write-Output`s a JSON array matching the `hello.sh` dynamic-list output format
- Update `docs/using/advanced/writing-scripts.md` with a Windows PowerShell example; document that scripts must be in `scripts/` and output UTF-8

#### Onboarding copy
- `paste_text` on Windows uses `SendInput` via `enigo` — no Accessibility permission is needed; gate the macOS Accessibility prompt text in `+page.svelte` so it does not appear on Windows

### Done when ✅
- Launcher builds, runs, and passes all tests on Windows 10/11
- `paste_text` and `copy_text` work end-to-end on Windows
- Window does not appear in the Windows taskbar while visible
- `hello.ps1` is seeded on first launch on Windows

---

## Stage 26 — Cross-Platform CI & Packaging ✅

### Goal
Automated CI that builds, tests, and packages the app on macOS, Linux, and Windows so platform regressions are caught before shipping.

### Tasks

#### CI workflow (`.github/workflows/ci.yml`)
- Matrix: `[macos-latest, ubuntu-22.04, windows-latest]`
- Each job: install Rust stable → install Node.js 20 → `npm ci` → `cargo test` → `npm run tauri build`
- Linux pre-step: install system deps (`libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `xdotool`)
- Cache `~/.cargo/registry` and `node_modules` per OS runner

#### Packaging
- macOS: `.dmg` produced by `tauri build`; note notarisation and code-signing as a future hardening step
- Linux: `.AppImage` and `.deb`; document that `xdotool` is a runtime dependency
- Windows: `.msi` (NSIS) installer; document UAC/SmartScreen first-run prompt

#### Documentation
- Add "Building from source" platform matrix to `docs/development-setup.md`
- Add platform-specific install instructions to `README.md`

### Done when ✅
- CI passes green on all three runners
- Build artefacts (`.dmg`, `.AppImage`, `.msi`) are produced successfully
- README documents how to build on each platform

---

## Stage 27 — Co-Located Resources ✅

### Goal
Eliminate the separate `lists/` and `scripts/` config subdirectories. Static list files and scripts now live **next to their command YAML** inside `commands/`, resolved via the command file's parent directory. This is a **breaking change** — existing users must reorganise their config directory.

### Breaking change

Before this stage, the config directory looked like:

```
Nimble/
├── settings.yaml
├── commands/
│   └── examples/
│       ├── show-team-emails.yaml
│       ├── dynamic-list-example.yaml
│       └── script-action-example.yaml
├── lists/
│   └── team-emails.yaml
└── scripts/
    ├── hello.sh
    ├── team-emails.sh
    └── timestamp.sh
```

After this stage:

```
Nimble/
├── settings.yaml
└── commands/
    └── examples/
        ├── show-team-emails/
        │   ├── show-team-emails.yaml
        │   └── team-emails.yaml
        ├── say-hello/
        │   ├── say-hello.yaml
        │   └── hello.sh
        ├── copy-uuid/
        │   ├── copy-uuid.yaml
        │   └── uuid.sh
        └── open-google.yaml          # simple commands stay as standalone files
```

The `lists/` and `scripts/` top-level directories are **no longer used or watched**. Commands that reference a list or script must place those files in the same directory as the command YAML (or a subdirectory of it).

### Advantages

- **Self-contained commands:** A command and all its resources live in a single folder — easy to copy, share, back up, or delete as a unit.
- **No naming collisions:** Two commands can use a script called `fetch.sh` without conflicting, since each resolves from its own directory.
- **Simpler mental model:** Users don't need to know about three separate locations (`commands/`, `lists/`, `scripts/`). Everything lives under `commands/`.
- **Easier sharing:** A command folder can be zipped and sent to another user — no need to remember to include files from two other directories.
- **Reduced watcher scope:** Only one directory tree (`commands/`) needs to be watched for changes, not three.

### Changes

#### Rust backend (`commands.rs`)
- Added `source_dir: String` field to the `Command` struct, populated at load time from the parent directory of the YAML file (relative to `commands/`).
- `load_list()` now accepts `command_dir` parameter and resolves list files relative to `config_dir/commands/<command_dir>/` instead of `config_dir/lists/`.
- `run_script()` and `run_script_values()` now accept `command_dir` parameter and resolve scripts relative to `config_dir/commands/<command_dir>/` instead of `config_dir/scripts/`.
- Security: plain filename validation (no `/`, `\`, `..`) still enforced for list and script names.
- Seed files reorganised: commands with scripts/lists are seeded into subdirectories (e.g., `commands/examples/say-hello/hello.sh` alongside `say-hello.yaml`).

#### Rust backend (`lib.rs`)
- `load_list`, `run_dynamic_list`, and `run_script_action` Tauri commands now accept a `command_dir: String` parameter, resolved as `config_dir/commands/<command_dir>/`.

#### Watcher (`watcher.rs`)
- Removed watching of `config_dir/lists/` and `config_dir/scripts/`.
- Only `config_dir/commands/` is watched recursively.
- Event filter updated to trigger on script file extensions (`.sh`, `.py`, `.js`, `.ps1`, `.bat`) in addition to `.yaml`/`.yml`.
- Removed seed script constants (`HELLO_SH`, `HELLO_PS1`) — seed scripts are now handled by `commands.rs`.

#### Frontend (`types.ts`, `+page.svelte`)
- Added `source_dir: string` to the `Command` TypeScript interface.
- All `invoke()` calls for `load_list`, `run_dynamic_list`, and `run_script_action` now pass `commandDir: command.source_dir` so the backend resolves resources from the correct directory.

#### Docs
- Updated `docs/using/advanced/static-list.md`, `dynamic-list.md`, `script-action.md`, `writing-scripts.md` to reflect co-located file resolution.
- Updated `docs/using/config-directory.md` to remove `lists/` and `scripts/` sections; documented the new co-location pattern.
- Updated `docs/using/configuring-commands.md` with the new directory layout.

#### Example config (`example-config/`)
- Removed `example-config/lists/` and `example-config/scripts/` directories.
- Commands that use lists or scripts are now subdirectories containing both the YAML and the resource files.
- Updated `example-config/README.md` to reflect the new structure.

#### Backend tests
- All existing tests updated to pass `command_dir` to `load_list`, `run_script`, and `run_script_values`.
- Test fixtures reorganised to match co-located structure.
- All 57 tests pass.

### Done when ✅
- `lists/` and `scripts/` top-level directories are no longer referenced by any code
- Static list files resolve from the command YAML's parent directory
- Scripts for `dynamic_list` and `script_action` resolve from the command YAML's parent directory
- File watcher only watches `commands/` recursively
- All 57 backend tests pass
- Docs, examples, and seed files reflect the new co-located layout

---

## Stage 28 — Built-In Script Environment Variables

### Goal
Expose app-provided runtime context to all scripts (`dynamic_list` and `script_action`) through built-in environment variables, so scripts can read context, paths, and platform metadata without relying on positional arguments.

### Proposed built-in variables

- `NIMBLE_CONTEXT` — active context string (or empty)
- `NIMBLE_PHRASE` — command phrase that triggered the script
- `NIMBLE_CONFIG_DIR` — absolute app config directory path
- `NIMBLE_COMMAND_DIR` — absolute directory containing the command YAML
- `NIMBLE_OS` — `macos`, `linux`, or `windows`
- `NIMBLE_VERSION` — app version

### Tasks

#### Backend script execution
- Add a small helper in `src-tauri/src/commands.rs` to inject a map of built-in env vars into `std::process::Command` for both `run_script()` and `run_script_values()`.
- Ensure Windows PowerShell execution path (`.ps1`) receives the same environment values as other executables.
- Keep variable names uppercase with `NIMBLE_` prefix.

#### Data flow
- Extend Tauri command inputs (`run_dynamic_list`, `run_script_action`) so frontend can pass active context and triggering phrase to backend.
- Thread these values into `commands::run_script()` / `commands::run_script_values()`.

#### Safety and compatibility
- Built-ins are informational only; no secret material is injected.
- Preserve existing timeout, stderr logging, and output parsing behavior.
- Built-in keys are reserved and non-overridable by user-defined env.

#### Docs and examples
- Add a section to `docs/using/advanced/writing-scripts.md` listing each built-in variable and an example script reading them.
- Update `docs/using/advanced/dynamic-list.md` and `docs/using/advanced/script-action.md` with references to built-in env usage.

### Done when
- Every script invocation receives the same `NIMBLE_*` environment variables across macOS, Linux, and Windows
- `dynamic_list` and `script_action` scripts can read active context and command metadata without extra args
- Built-in variable behavior is documented in the advanced script docs

---

## Stage 29 — User-Defined Script Variables

### Goal
Allow users to define their own environment variables for scripts globally and per command scope, with explicit precedence and clear override behavior.

### Scope model

#### Global variables
- Add optional `env.yaml` at config root: `Nimble/env.yaml`
- File shape: flat map of `KEY: value` pairs (string values)

#### Command-scoped variables
- Support optional `env.yaml` in the command YAML directory (`source_dir/env.yaml`) only
- No directory walking; only same-directory sidecar is considered
- Support optional inline `env:` block in command YAML for command-specific overrides

### Precedence order (lowest -> highest)

```text
System environment
NIMBLE_* built-ins
Global Nimble/env.yaml
source_dir/env.yaml
Command inline env:
```

Built-in `NIMBLE_*` keys remain reserved and cannot be overridden.

### Tasks

#### Backend loading and validation
- Add helpers in `src-tauri/src/commands.rs` to load and merge user-defined env layers.
- Validate keys: non-empty, portable format (`[A-Z_][A-Z0-9_]*`), and reject reserved prefix `NIMBLE_`.
- Treat missing env files as empty; malformed files return clear, non-fatal errors (script call fails with message).

#### Execution integration
- Apply merged env layers to both `run_script()` and `run_script_values()`.
- Ensure behavior is identical for `dynamic_list` and `script_action`.

#### Docs and examples
- Add `env.yaml` sections to `docs/using/config-directory.md` and `docs/using/advanced/writing-scripts.md`.
- Add example command folders in `example-config/` demonstrating:
  - global env only
  - same-directory sidecar env
  - inline `env:` override

#### Tests
- Unit tests for merge precedence and reserved-key rejection.
- Tests confirming same-directory-only sidecar behavior (no parent traversal).
- Tests that built-ins are still present and non-overridable after user env merge.

### Done when
- Users can define script variables globally and per-command scope
- Precedence is deterministic and documented
- Reserved `NIMBLE_*` names are protected
- All script execution paths honor the same env merge rules

---

## Stage 30 — Script Debugging & Verbose Logs

### Goal
Introduce an explicit script debug mode with predictable diagnostics, including a built-in `NIMBLE_DEBUG` environment variable for scripts and improved launcher-side logging for script execution issues.

### Scope

- Add `NIMBLE_DEBUG=1` when debug mode is enabled.
- Keep Stage 28 minimal set unchanged; `NIMBLE_DEBUG` is intentionally delivered in this stage.
- Apply to both `dynamic_list` and `script_action` execution paths.

### Tasks

#### Settings and toggles
- Add `script_debug: bool` to `settings.yaml` (default `false`).
- Load this flag through existing settings flow and pass it into script execution calls.

#### Environment variable behavior
- Inject `NIMBLE_DEBUG=1` only when `script_debug` is enabled.
- Omit `NIMBLE_DEBUG` entirely when debug mode is disabled (rather than setting `0`).

#### Logging behavior
- In debug mode, log script invocation metadata: command phrase, script path, arg mode, argument presence, and execution duration.
- Keep stderr capture behavior, but include clearer labels and context for script failures/timeouts.
- Ensure logs never include sensitive user-defined variable values.

#### Docs
- Update `docs/using/advanced/writing-scripts.md` with a debug section and examples (`if [ -n "$NIMBLE_DEBUG" ]; then ... fi`).
- Document `script_debug` in `docs/using/config-directory.md` / `settings.yaml` reference.

#### Tests
- Unit tests verifying `NIMBLE_DEBUG` is set only when debug mode is true.
- Tests ensuring debug toggle affects both `run_script()` and `run_script_values()` paths.

### Done when
- Users can enable script debug mode via settings
- Scripts receive `NIMBLE_DEBUG=1` only in debug mode
- Verbose script diagnostics are available without changing script behavior in normal mode

## Stage 31 — External Script/List Paths ✅

### Goal
Allow `script:` and `list:` fields to reference files outside the command directory via `${VAR}` token substitution, with a safety toggle (`allow_external_paths`) in `settings.yaml`.

### Scope

- Add `${VAR}` substitution to `script:` and `list:` field values.
- Variables are resolved against built-in `NIMBLE_*` vars first, then user-defined env.
- Plain filenames (no `${…}`) retain the legacy co-located behaviour unchanged.
- Add `allow_external_paths: bool` to `settings.yaml` (default `true`).
- When `false`, resolved paths must stay inside the command directory.

### Tasks

#### Backend — `commands.rs`
- `substitute_vars(template, env)` — replaces `${VAR}` tokens; errors on undefined/unterminated.
- `builtin_var_value(name, env)` — returns `NIMBLE_CONTEXT`, `NIMBLE_PHRASE`, `NIMBLE_OS`.
- `resolve_script_path(raw, command_dir, env)` — resolves `script:` value; pre-substitutes `NIMBLE_CONFIG_DIR`, `NIMBLE_COMMAND_DIR`, `NIMBLE_VERSION`; enforces containment when `allow_external_paths` is `false`.
- `resolve_list_path(raw, command_dir, env)` — same for `list:` value; auto-appends `.yaml`.
- Updated `run_script()`, `run_script_values()`, `load_list()` to use the new resolvers.
- Added `allow_external_paths` field to `ScriptEnv`.

#### Backend — `settings.rs`
- Added `allow_external_paths: bool` (default `true`) to `AppSettings`.

#### Backend — `lib.rs` (Tauri handlers)
- `load_list` handler now accepts `inline_env`, `context`, `phrase` and reads `allow_external_paths` from `SettingsState`.
- `run_dynamic_list` and `run_script_action` pass `allow_external_paths` through `ScriptEnv`.

#### Frontend
- Updated both `load_list` invoke calls with `inlineEnv`, `context`, `phrase`.
- Added `allow_external_paths: boolean` to `AppSettings` TypeScript interface.

#### Tests (19 new, 114 total)
- `substitute_vars`: plain text, single builtin, multiple vars, user env, undefined error, unterminated error, empty name error, builtin-overrides-user.
- `resolve_script_path`: plain name, rejects slash, rejects dotdot, var expansion absolute, external blocked, command_dir var when external false.
- `resolve_list_path`: plain name appends yaml, rejects slash, var with yaml ext, auto-append yaml, external blocked.
- `settings.rs`: `can_disable_allow_external_paths`, malformed yaml includes default.

#### Docs
- `writing-scripts.md`: updated security section; added "External scripts and lists" section.
- `configuring-commands.md`: schema updated with `${VAR}` comments.
- `config-directory.md`: `settings.yaml` section includes `allow_external_paths`.
- `dynamic-list.md`, `script-action.md`, `static-list.md`: brief cross-references.

#### Examples
- `example-config/scripts/greeting.sh` — shared script.
- `example-config/commands/examples/shared-greeting.yaml` — dynamic_list using `${NIMBLE_CONFIG_DIR}`.
- `example-config/settings.yaml` updated with `allow_external_paths: true`.

### Done when
- `${VAR}` tokens in `script:` and `list:` fields resolve correctly.
- External paths are allowed by default and can be restricted via `allow_external_paths: false`.
- 114 tests pass; no regressions.

---

## Stage 32 — Copilot Agents ✅

### Goal
Provide two specialised GitHub Copilot agents that help users create Nimble commands and write scripts directly from their editor, without needing to memorise the YAML schema or script output formats.

### Agents

#### `@nimble-command`
- **Purpose:** Create, edit, and debug YAML command files
- **Knowledge:** Full command schema, all six action types, environment variable layering, co-location rules, `${VAR}` substitution, context matching
- **Constraints:** Only creates/edits YAML command files and list files; delegates to `@nimble-script` when a script is needed; never modifies Rust source or frontend code
- **Location:** `.github/agents/nimble-command.agent.md`

#### `@nimble-script`
- **Purpose:** Write, debug, and improve scripts for `dynamic_list` and `script_action` commands
- **Knowledge:** Output formats (JSON array for dynamic_list, JSON string array for script_action), argument passing by language, built-in `NIMBLE_*` env vars, timeout constraints, platform differences (shebang, chmod, PowerShell)
- **Constraints:** Only creates/edits script files; never modifies YAML commands, env.yaml, or settings.yaml; never runs scripts with elevated privileges
- **Location:** `.github/agents/nimble-script.agent.md`

### Architecture decision
A what-if analysis (`.vibe/what-ifs/command-authoring-agents/summary.md`) evaluated single-agent vs multi-agent approaches. Two focused agents were chosen over a single agent with mode routing — each agent has a clear, non-overlapping scope and can delegate to the other, keeping instructions concise and reliable.

### Documentation
- Added `docs/using/advanced/copilot-agents.md` with usage examples, agent boundaries, and tips
- Updated `docs/using/advanced/README.md` and `docs/using/README.md` landing pages with links

### Done when ✅
- Both agent definition files exist in `.github/agents/`
- Each agent covers its full domain (command schema / script formats) with examples and constraints
- User-facing documentation describes how to invoke each agent and what it can do

---

## Stage 33 — Branding & App Identity ✅

**Goal:** Replace placeholder icons with the chosen D3 Warm Neon brand design and establish the final bundle identifier across all platform manifests.

### Tasks
- Move `nimble-icons.html` icon concepts file to `.vibe/icons/` for reference
- Create `.vibe/icons/icon.svg` — 1024×1024 master SVG from the D3 Warm Neon concept (fuchsia→violet stacked action cards on deep aubergine background)
- Run `tauri icon` to generate all platform assets from the master SVG:
  - macOS: `icon.icns`
  - Windows: `icon.ico`, `StoreLogo.png`, `Square*.png` (10 sizes for MSIX/Appx)
  - Linux: `icon.png`, `32x32.png`, `64x64.png`, `128x128.png`, `128x128@2x.png`
  - iOS: 18 `AppIcon-*.png` variants
  - Android: `mipmap-*` foreground, round, and launcher PNGs
- Update `tauri.conf.json`:
  - Set `identifier` to `io.switchpanel.nimble`
  - Add `64x64.png` to icon list
- Update `flatpak/app.nimble.App.json` `app-id` to `io.switchpanel.nimble`

### Done when ✅
- `.vibe/icons/icon.svg` exists as the single source of truth for the app icon
- All icons in `src-tauri/icons/` are regenerated from the D3 Warm Neon design
- Bundle identifier is `io.switchpanel.nimble` in `tauri.conf.json` and `flatpak/*.json`
- App builds and tests pass with the new identity

---

## Stage 34 — Agent Spec Refactor ✅

**Goal:** Extract all schema knowledge embedded inline in the Copilot agent definitions into a single, canonical machine-readable spec file so that agents stay in sync with the codebase as the schema evolves.

### Problem
Both `nimble-command.agent.md` (~145 lines) and `nimble-script.agent.md` (~200 lines) contained full copies of the command YAML schema, action type details, environment variable tables, script output formats, and platform invocation rules. When the schema changed, every agent file had to be updated manually — a maintenance burden and a source of drift.

### Solution
Introduced a layered discovery pattern:

1. **`nimble-spec.yaml`** — the single source of truth for the entire Nimble schema: all 6 action types with field definitions, the script interface (output formats, argument passing, timeout, platform invocation), the 6 built-in `NIMBLE_*` env vars, 3-layer user-defined env var precedence, settings schema, contexts, and co-location patterns. Includes a `changelog` section at the bottom so agents can detect drift.
2. **`nimble-conventions.md`** — shared agent rules covering file placement, reserved phrases, testing requirements, agent boundaries, and the spec update process.
3. **Thin agent pointers** — both agent `.md` files were refactored to remove all inline schema and replaced with a "Bootstrap" section that instructs the agent to read the spec file first before answering any questions.
4. **Rule 12a in `copilot-instructions.md`** — enforces that any change to the command YAML schema, action types, settings, script interface, env var API, or context system must also update `nimble-spec.yaml` and add a changelog entry.

### Files created
- `.github/agents/nimble-spec.yaml` — canonical spec (v1)
- `.github/agents/nimble-conventions.md` — shared agent conventions

### Files modified
- `.github/agents/nimble-command.agent.md` — removed ~120 lines of inline schema, added spec bootstrap
- `.github/agents/nimble-script.agent.md` — removed ~150 lines of inline schema, added spec bootstrap
- `.github/copilot-instructions.md` — added rule 12a (keep spec in sync)

### Done when ✅
- `nimble-spec.yaml` exists and covers all 6 action types, script interface, env vars, settings, and contexts
- Both agent files bootstrap from the spec instead of embedding schema inline
- `copilot-instructions.md` includes a rule requiring spec updates on schema changes
- All 114 Rust tests pass

---

## Stage 35 — Static List TSV Format ✅

**Goal:** Change the static list file format from YAML to TSV (tab-separated values) for easier human editing. This is a breaking change — existing list files must be converted.

### Motivation

YAML list files (`- title: ... subtext: ...`) were verbose for what is essentially a two-column table. TSV eliminates all syntax overhead — one item per line, tab between columns. Additionally, co-located YAML list files caused parse-error noise in logs because the command loader tried to parse them as commands. TSV files use a `.tsv` extension, which the loader naturally skips.

See the what-if analyses in `.vibe/what-ifs/` for the full decision process:
- `command-filename-convention/` — why log noise is better solved by format separation than filename heuristics
- `static-list-tsv-format/` — TSV vs YAML with extensibility analysis
- `static-list-tsv-vs-csv/` — head-to-head TSV vs CSV; TSV wins because commas appear naturally in data

### Changes

#### New TSV list format
```
# Team email addresses
Alice Smith	alice@example.com
Bob Jones	bob@example.com
Carol White	carol@example.com
```

Rules:
- One item per line; tab separates title from optional subtext
- Lines starting with `#` are comments; blank lines are ignored
- No quoting or escaping needed — commas, quotes, and special characters in data just work
- If a line has no tab, the entire line is the title (subtext is absent)

#### Rust backend
- `load_list()` — replaced `serde_yaml::from_str` with a new `parse_tsv_list()` function
- `resolve_list_path()` — appends `.tsv` instead of `.yaml`; `${VAR}` paths auto-append `.tsv`
- `parse_tsv_list()` — new function: splits lines on tab, skips `#` comments and blank lines
- Seed file changed from `team-emails.yaml` (YAML) to `team-emails.tsv` (TSV)

#### Tests
- Existing list tests updated to use TSV content
- Added: `load_list_skips_comments_and_blank_lines`, `load_list_title_with_comma_works`
- `resolve_list_path` tests assert `.tsv` extension
- 116 total tests (2 new), all passing

#### Documentation
- `docs/using/advanced/static-list.md` — format section rewritten for TSV
- `docs/using/configuring-commands.md` — schema comment updated (`.tsv` extension)
- `docs/using/config-directory.md` — example layout updated
- `example-config/` — `team-emails.yaml` replaced with `team-emails.tsv`
- `example-config/README.md` — directory tree updated
- `.github/agents/nimble-spec.yaml` — `list_file_format`, `co_location`, changelog updated

### Migration

Existing YAML list files must be converted to TSV:
```
# Before (team-emails.yaml)
- title: Alice Smith
  subtext: alice@example.com
- title: Bob Jones
  subtext: bob@example.com

# After (team-emails.tsv)
Alice Smith	alice@example.com
Bob Jones	bob@example.com
```

### Done when ✅
- `load_list()` parses TSV format; `resolve_list_path()` resolves to `.tsv`
- Seed files and example config use TSV
- All docs and spec updated to reflect TSV format
- 116 Rust tests pass

---

## Stage 36 — Docs Restructure ✅

### Goal
Replace the `docs/using/` directory (with its `basic/` and `advanced/` skill-level split) with a clearer taxonomy: `docs/actions/`, `docs/guides/`, `docs/reference/`. Make documentation easy to navigate for new users and clearly distinguish action type docs from workflow/feature docs.

### Problem
- The `using/` folder name was vague — users had to click into it to understand what it contained.
- The `basic/` vs `advanced/` split grouped docs by perceived difficulty rather than by what they described. `advanced/` mixed action type references (static-list, dynamic-list, script-action) with workflow guides (writing-scripts, contexts, copilot-agents).
- New users had no clear starting point that covered both onboarding and first command setup.

### Changes

#### New directory structure
```
docs/
├── getting-started.md         ← renamed from using/first-run.md; covers onboarding + first command
├── actions/                   ← all 6 action types in one flat directory
│   ├── README.md              ← hub page: table of all 6 actions + execution methods
│   ├── open-url.md
│   ├── paste-text.md
│   ├── copy-text.md
│   ├── static-list.md
│   ├── dynamic-list.md
│   └── script-action.md
├── guides/                    ← workflow/feature documentation
│   ├── README.md              ← hub page: table of all guides
│   ├── configuring-commands.md
│   ├── writing-scripts.md
│   ├── contexts.md            ← renamed from context.md
│   └── copilot-agents.md
├── reference/                 ← lookup material
│   ├── README.md              ← hub page: table of all reference docs
│   ├── config-directory.md
│   ├── duplicate-commands.md
│   └── tips-and-tricks.md
├── development-setup.md       ← unchanged
├── motivation.md              ← unchanged
└── roadmap.md                 ← unchanged
```

#### Files moved (via `git mv` to preserve history)
- `using/basic/{open-url,paste-text,copy-text}.md` → `actions/`
- `using/advanced/{static-list,dynamic-list,script-action}.md` → `actions/`
- `using/{configuring-commands}.md` → `guides/`
- `using/advanced/{writing-scripts,context→contexts,copilot-agents}.md` → `guides/`
- `using/{config-directory,tips-and-tricks,duplicate-commands}.md` → `reference/`
- `using/first-run.md` → `getting-started.md` (at docs root)

#### Files deleted
- `docs/using/README.md`, `docs/using/basic/README.md`, `docs/using/advanced/README.md` (replaced by new hub pages)

#### Files created
- `docs/actions/README.md` — hub page listing all 6 action types
- `docs/guides/README.md` — hub page listing all guides
- `docs/reference/README.md` — hub page listing all reference docs

#### Cross-links updated
- All internal doc links updated to reflect new paths
- Root `README.md` docs table updated
- `.github/copilot-instructions.md` rules 6–8 updated
- `.github/prompts/new-action.prompt.md` step 8 updated
- `docs/motivation.md` context link updated
- Stale `[ctx]` log prefix in `duplicate-commands.md` example updated to `[nimble]`

### Done when ✅
- All docs accessible at their new paths with no broken cross-links
- Root README.md docs table reflects the new structure
- Copilot instructions and prompt files reference the new paths
- Old `docs/using/` directory fully removed
- 116 Rust tests still pass (no code changes)

---

## Stage 37 — UI Polish & Window Dragging ✅

**Goal:** Make the launcher visually refined and allow the user to reposition the window on screen.

### Changes

#### Window dragging
- Added `data-tauri-drag-region` to the `.launcher` container for non-interactive areas
- Added `onmousedown={() => appWindow.startDragging()}` on the `<input>` element so clicking and holding on the input initiates a native drag
- Added `core:window:allow-start-dragging` permission to Tauri capabilities

#### Prompt glyph
- Replaced bare text input with a `»` (double chevron) glyph on the left of the input row, conveying "command input ready"
- Evaluated alternatives: magnifying glass, hummingbird, diamond, `>_` prompt — settled on double chevron for its minimal, clean feel

#### Backdrop blur (vibrancy)
- `.launcher` background opacity reduced from `0.95` to `0.82`
- Added `-webkit-backdrop-filter: blur(40px) saturate(1.8)` for frosted-glass effect on macOS

#### Layered shadow stack
- Replaced single heavy shadow with a 3-layer stack: subtle border (`0 0 0 1px`), mid-spread (`0 8px 24px`), deep ambient (`0 24px 64px`)

#### Accent selection indicator
- Selected result rows show a 3px left border in accent blue (`#0a84ff`) instead of only a white-overlay background

#### Action-type badges
- Each command result row displays a small uppercase pill on the right: `URL`, `Paste`, `Copy`, `List`, or `Script`
- Badge styling adjusts slightly when the row is selected

#### Subtle row scale
- Selected rows receive `transform: scale(1.005)` for a hint of physical lift

#### Footer hint bar (added then removed)
- Briefly added a `↵ Run  ⇥ Complete  ⎋ Close` footer bar
- Removed after review: hints were inaccurate in list mode (Return selects a list item, not "Run") and added visual noise without enough value

### Done when ✅
- Window is draggable by clicking and holding the input area or any non-interactive launcher region
- Launcher displays prompt glyph, frosted-glass background, layered shadows, accent selection border, action badges, and subtle row scale
- No footer hint bar is shown
- 116 Rust tests still pass

---

## Stage 38 — Spec & Agent Versioning ✅

**Goal:** Introduce independent versioning for `nimble-spec.yaml` and agent files so the schema/API contract is explicitly tracked separately from the app version.

### Changes

- Fixed changelog: TSV format change entry bumped to `version: 2` (was duplicate `version: 1`)
- Added `spec_version: 2` to `nimble-command.agent.md` and `nimble-script.agent.md` frontmatter
- Broadened bump rules in `copilot-instructions.md` rule 12a: now covers additive changes (new action types, env vars, config fields), not just removals/type changes
- Updated `nimble-conventions.md` with expanded bump criteria and agent sync requirement
- Published spec and agents to public repo (`surdy/nimble`)

### Done when ✅
- `nimble-spec.yaml` changelog has unique version numbers per entry
- Both agent files declare `spec_version` in frontmatter matching the current spec
- Bump rules cover additive changes, not just breaking ones
- Conventions doc and copilot instructions are updated
- Spec and agents published to public repo

---

## Stage 39 — `/nimble docs` Built-in Command ✅

**Goal:** Give users an in-launcher way to open documentation pages in the browser, starting with a deploying-agents guide for Copilot agent setup.

### Changes

- Added `docs_open` action variant to `BuiltinConfig` in `types.ts` (with optional `url` field)
- Added five `/nimble docs` built-in commands: `agents`, `commands`, `scripts`, `actions`, `contexts`
- Each opens the corresponding GitHub docs page in the default browser and dismisses the launcher
- Added `Action` import to `+page.svelte` for type-safe badge function
- Refactored `actionBadge()` to accept a `Command`-shaped object (shows "Docs" badge for docs commands)
- Fixed pre-existing `svelte-check` error: added missing `env` and `source_dir` fields to all built-in commands
- Created `docs/guides/deploying-agents.md` — step-by-step guide with `curl` commands
- Updated `docs/guides/README.md` to link the new guide
- Published deploying-agents guide and updated README to public repo

### Done when ✅
- Typing `/nimble` shows all five docs commands with "Docs" badge
- Selecting a docs command opens the corresponding page in the browser
- `svelte-check` passes with zero errors
- 116 Rust tests pass
- Deploying-agents guide published to public repo

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

## Stage 2 — Command Data Model

**Goal:** Define what a command looks like in code and ship a small set of hard-coded example commands.

### Tasks
- Define the command schema:
  ```
  {
    phrase: string          // the multi-word command phrase (e.g. "open google")
    title: string           // human-readable description shown as the result title
    action: {
      type: "open_url" | "paste_text"
      config: { ... }       // action-specific fields
    }
  }
  ```
- Create a static in-memory command store with 3–5 example commands covering both action types
- Expose the store to the frontend via a Tauri command

### Done when
- A Tauri command returns a list of example commands to the frontend

---

## Stage 3 — Partial Matching & Results UI

**Goal:** As the user types, filter commands in real time and display matching results.

### Tasks
- Implement partial/prefix matching: a typed string matches any command whose phrase contains it as a contiguous substring (case-insensitive)
- Display each match as a result row:
  - **Title** (main line): the command's `title` field
  - **Subtext** (secondary line): the full command phrase, showing what the user's input completes to
- Highlight the matching portion of the phrase in the subtext
- Keyboard navigation: Up/Down arrows to move selection, Enter to confirm
- Show a "no results" state when nothing matches

### Done when
- Typing partial phrases filters the list live; keyboard navigation and selection work

---

## Stage 4 — Action: Open URL

**Goal:** Executing a selected command with type `open_url` opens a URL in the default browser.

### Tasks
- Implement the `open_url` built-in action in the Rust/Tauri layer
- Support an optional `param` variable: if the config URL contains `{param}`, substitute the text the user typed after the command phrase
  - Example: command phrase `"search google"`, URL `"https://google.com/search?q={param}"`, user types `"search google rust programming"` → opens `https://google.com/search?q=rust+programming`
- Validate the URL before opening (must be http/https); reject anything else
- After executing, close the launcher window

### Done when
- Selecting an open_url command opens the correct URL in the browser; param substitution works; window closes

---

## Stage 5 — Action: Paste Text

**Goal:** Executing a selected command with type `paste_text` pastes a predefined string into the app that had focus before the launcher was invoked.

### Tasks
- Record which application/window had focus immediately before the launcher window was shown
- Implement the `paste_text` built-in action in the Rust/Tauri layer:
  1. Close/hide the launcher window
  2. Restore focus to the previously active application
  3. Write the configured text to the system clipboard
  4. Programmatically trigger a paste (Cmd+V on macOS)
- Sanitise the text to paste (plain text only; no executable content)

### Done when
- Selecting a paste_text command inserts the configured text at the cursor position in the previously focused app

---

## Stage 6 — Global Hotkey

**Goal:** The user can open and dismiss the launcher from any application using a keyboard shortcut.

### Tasks
- Register a global hotkey (default: `Cmd+Space` or a configurable alternative) using Tauri's global shortcut plugin
- Pressing the hotkey while the launcher is hidden: show and focus it
- Pressing the hotkey while the launcher is visible (or pressing Escape): hide it and restore focus to the previous app
- Ensure the hotkey is released cleanly when the app quits

### Done when
- The launcher can be summoned and dismissed system-wide without switching apps manually

---

## Stage 7 — User Command Configuration

**Goal:** Commands are loaded from a user-editable config file rather than being hard-coded.

### Tasks
- Choose a config format (JSON or TOML — decision to be made)
- Define the config file location: platform-appropriate app config directory (e.g. `~/Library/Application Support/contexts-launcher/commands.json` on macOS)
- Load commands from the config file on startup; fall back to built-in examples if no file exists
- Watch the config file for changes and hot-reload commands without restarting the launcher
- Document the config file format in `docs/`

### Done when
- A user can add, edit, or remove commands by editing the config file and see changes reflected immediately

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
| 2 | Command data model | Typed schema + hard-coded example commands |
| 3 | Partial matching & results UI | Live filtering, keyboard navigation, title/subtext display |
| 4 | Action: Open URL | Opens URLs in browser, supports `{param}` substitution |
| 5 | Action: Paste Text | Pastes text into previously focused application |
| 6 | Global hotkey | System-wide shortcut to summon/dismiss launcher |
| 7 | User config file | Commands loaded from and hot-reloaded from a config file |
| 8 | Script extensions | External scripts return structured results; launcher executes built-in actions |

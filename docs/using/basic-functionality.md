# Basic Functionality

Ctx currently supports three built-in actions: **Open URL**, **Paste Text**, and **Copy Text**. Every command you define in a YAML file must use one of these action types.

---

## Open URL

Opens a URL in your default browser when the command is executed.

### Minimal example

```yaml
phrase: open github
title: Open GitHub
action:
  type: open_url
  config:
    url: https://github.com
```

When you type `open github` (or any substring of it) in the launcher and press `Enter` or click the result, your default browser opens `https://github.com`.

### With parameter substitution

Add `{param}` anywhere in the URL to capture extra text the user types after the command phrase.

```yaml
phrase: search google
title: Search Google
action:
  type: open_url
  config:
    url: https://www.google.com/search?q={param}
```

Typing `search google rust programming` opens `https://www.google.com/search?q=rust+programming`. The text after the matched phrase is URL-encoded automatically.

### Supported URL schemes

Ctx accepts any valid URL scheme — not just `http` and `https`. This means deep links for desktop apps (e.g. `slack://`, `obsidian://`) and other protocols (e.g. `mailto:`, `tel:`) all work out of the box. See [Tips & Tricks](tips-and-tricks.md) for examples.

---

## Paste Text

Pastes a predefined block of text into whichever application had focus before you opened the launcher.

### How it works

1. You invoke the launcher and select a `paste_text` command
2. Ctx hides the launcher window and restores focus to your previous app
3. After a brief delay (80 ms), Ctx writes your configured text to the clipboard and simulates `⌘V`
4. The text appears in your previously focused app as if you had pasted it yourself

### Example

```yaml
phrase: paste email signature
title: Paste email signature
action:
  type: paste_text
  config:
    text: |
      Best regards,
      Jane Smith
      jane@example.com | +1 555 0100
```

### macOS Accessibility permission

Ctx uses macOS Accessibility APIs to simulate the paste keystroke. The first time you run a `paste_text` command, macOS will prompt you to grant Accessibility access in **System Settings → Privacy & Security → Accessibility**. Without this permission the keystroke simulation is blocked and the text will not be pasted.

---

## Copy Text

Copies a predefined block of text to the clipboard. The launcher dismisses immediately — no paste occurs. You paste the text yourself wherever you need it.

### How it differs from Paste Text

| | `paste_text` | `copy_text` |
|---|---|---|
| Writes to clipboard | ✅ | ✅ |
| Simulates ⌘V / Ctrl+V | ✅ | ❌ |
| Requires Accessibility permission | ✅ | ❌ |
| You paste manually | ❌ | ✅ |

Use `copy_text` when you want the text on the clipboard but prefer to control where it goes, or when the target app blocks simulated keystrokes.

### Example

```yaml
phrase: copy email
title: Copy email address
action:
  type: copy_text
  config:
    text: hello@example.com
```

---

## Executing a command

Three equivalent ways to run the selected command:

| Method | Description |
|--------|-------------|
| `Enter` | Executes the currently highlighted result |
| `↑` / `↓` | Move the highlight; `Enter` to confirm |
| **Click** | Click any result row to execute it immediately |

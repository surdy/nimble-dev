# Basic Functionality

Context Actions provides three built-in actions for everyday use. Each is configured in a YAML file placed in your `commands/` directory.

| Action | What it does |
|--------|-------------|
| [Open URL](open-url.md) | Opens a URL in your default browser — supports parameter substitution |
| [Paste Text](paste-text.md) | Pastes a predefined block of text into your previously focused app |
| [Copy Text](copy-text.md) | Copies a predefined block of text to the clipboard |

For data-driven or script-powered actions, see [Advanced Features](../advanced/README.md).

---

## Executing a command

| Method | Description |
|--------|-------------|
| `Enter` | Executes the currently highlighted result |
| `↑` / `↓` | Move the highlight; `Enter` to confirm |
| **Click** | Click any result row to execute it immediately |

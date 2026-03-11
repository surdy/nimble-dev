# Writing Scripts

The `dynamic_list` and `script_action` actions can run any executable you place in the `scripts/` directory of your config folder.

---

## Overview

Scripts live in the `scripts/` subdirectory of your config directory. Any executable file can be used: shell scripts, Python programs, Node.js scripts, compiled binaries, etc. Context Actions spawns the script, captures its stdout, and renders the output as a list of selectable items.

---

## Writing a script

### Output format

Your script must write to **stdout** only. Stderr is captured and logged internally but never shown to the user.

**Option 1 — JSON array (recommended):**

Return a JSON array of objects. Each object must have a `title` field; `subtext` is optional.

```json
[
  { "title": "Alice Smith", "subtext": "alice@example.com" },
  { "title": "Bob Jones", "subtext": "bob@example.com" },
  { "title": "Carol White" }
]
```

**Option 2 — Plain text:**

Return a single line of text. Context Actions treats the entire trimmed output as the title of one item.

```
Hello, World!
```

**Empty output:** Return nothing (exit immediately) to show an empty list.

### Accepting an argument

When the command's `arg` mode is `optional` or `required`, Context Actions passes the user's typed suffix as the first positional argument (`$1` in shell, `sys.argv[1]` in Python, `process.argv[2]` in Node.js).

```sh
#!/bin/sh
# Filter contacts by the typed query (first argument)
QUERY="$1"
if [ -z "$QUERY" ]; then
  # No argument — return all contacts
  echo '[{"title":"Alice Smith","subtext":"alice@example.com"},{"title":"Bob Jones","subtext":"bob@example.com"}]'
else
  # Filter by query (simple grep example)
  echo '[{"title":"Alice Smith","subtext":"alice@example.com"}]'
fi
```

### Timeout

Context Actions enforces a **5-second timeout**. If the script does not exit within 5 seconds, an empty list is shown and the script process is abandoned. Keep scripts fast.

---

## Registering a script as a command

Create a YAML command file in `commands/` that references the script by filename:

```yaml
phrase: search contacts
title: Search contacts
action:
  type: dynamic_list
  config:
    script: contacts.sh     # resolves to scripts/contacts.sh
    arg: optional           # none | optional | required
    item_action: paste_text # optional; same as static_list
```

See [Dynamic List](dynamic-list.md) for the full YAML schema and argument mode reference.

---

## Argument modes

| `arg` value | When the list appears | Script receives |
|-------------|----------------------|--------------------|
| `none` *(default)* | Exact phrase match only | No arguments |
| `optional` | Exact match (immediately) **and** phrase + suffix | No arg on exact match; suffix otherwise |
| `required` | Only when a non-empty suffix follows the phrase | The typed suffix as `$1` |

**`none`** — good for static or slow-changing data that does not need filtering.  
**`optional`** — good for searchable lists where seeing all items first is useful.  
**`required`** — good for queries that only make sense with user input (e.g. search, lookup).

---

## Security boundaries

- `script` field values containing `/`, `\`, or `..` are **rejected** at invocation time. Scripts must be plain filenames inside `scripts/` — no subdirectories or path traversal.
- Scripts run with the **same user privileges** as the Context Actions launcher process. They are never elevated.
- Scripts **cannot** directly trigger launcher actions. They can only produce output. The launcher decides what to do with each item based on `item_action`.
- Script output is parsed and validated. Malformed JSON is treated as plain text; an entirely unparseable response shows an empty list.

---

## Debugging tips

1. **Run the script directly** from your terminal to see its output and any errors:
   ```sh
   ~/Library/Application\ Support/ContextActions/scripts/my-script.sh "test arg"
   ```

2. **Check stderr** — Context Actions logs scripts' stderr output with `[ctx] script "..." stderr:` prefix. Look in the app's log output (visible when running via `npm run tauri dev`).

3. **Validate JSON** — if items aren't appearing, paste your script's output into a JSON validator. Common issues: trailing commas, unescaped quotes, non-UTF-8 bytes.

4. **Check permissions** — on macOS/Linux the script file must be executable:
   ```sh
   chmod +x ~/Library/Application\ Support/ContextActions/scripts/my-script.sh
   ```

5. **Live reload** — editing any file in `scripts/` triggers a reload. If a dynamic list is currently displayed it re-runs automatically within the 300 ms debounce window.

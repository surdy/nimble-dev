# Config Directory Structure

Context Actions stores all per-user data in a single platform-specific config directory. The directory and its subdirectories are created automatically on first launch.

---

## Root directory location

| Platform | Path |
|----------|------|
| macOS | `~/Library/Application Support/ContextActions/` |
| Linux | `$XDG_CONFIG_HOME/ContextActions/` (falls back to `~/.config/ContextActions/`) |
| Windows | `%APPDATA%\ContextActions\` |

---

## Subdirectory layout

```
ContextActions/
  settings.yaml     ← application settings (hotkey, show_context_chip, allow_duplicates)
  commands/         ← YAML command files (watched and hot-reloaded by Context Actions)
    examples/       ← seeded on first launch if commands/ is empty
    …               ← your own files and subdirectories
  lists/            ← named list files for the static_list action type
  scripts/          ← executable scripts for the dynamic_list action type
```

Each subdirectory holds a distinct type of data. New subdirectories will be introduced in future releases as new features are added; each will be documented in this file.

---

## `settings.yaml`

The `settings.yaml` file at the root of the config directory controls application-level behaviour. It is created automatically the first time you run Context Actions.

```yaml
# hotkey: Super+Space   # uncomment and set your preferred global shortcut

# Show the active-context chip inside the launcher bar (default: true)
show_context_chip: true

# When false, the first file that defines a phrase wins and duplicate
# phrases in other files generate warnings. Default is true (all loaded).
allow_duplicates: true
```

**`hotkey`** — The global shortcut that summons the launcher from anywhere. You normally set this via the onboarding screen; editing it here manually is possible but requires a restart. Deleting this line triggers the onboarding screen on the next launch.

**`show_context_chip`** — When `true` (default), a pill badge showing the active context is displayed inside the launcher bar alongside a clear button. Set to `false` to hide it. Takes effect on next relaunch.

**`allow_duplicates`** — When `true` (default), all command files are loaded regardless of phrase conflicts. Set to `false` to enable first-file-wins deduplication and surface warnings for any conflicting phrases. Takes effect on next relaunch.

---

## `commands/`

Contains all YAML command files. Context Actions watches this subdirectory recursively and reloads commands within ~300 ms whenever a file is added, changed, or removed — no restart required.

You can organise your command files into any subdir structure you like:

```
commands/
  open-github.yaml
  search-google.yaml
  snippets/
    email-signature.yaml
    legal-disclaimer.yaml
  work/
    open-jira.yaml
    paste-standup-template.yaml
```

For the full command YAML schema, action types, and live-reload details see [Configuring Commands](configuring-commands.md).

---

## `lists/`

Contains named list files used by the `static_list` action type. Each file is a YAML array of items that the launcher can display inline when the corresponding command phrase is typed.

File names (without the `.yaml` extension) are how commands reference their list:
```yaml
action:
  type: static_list
  config:
    list: team-emails    # reads lists/team-emails.yaml
```

Changes to files in `lists/` are detected by the file watcher and will refresh any list currently displayed in the launcher within the ~300 ms debounce window.

For the full list file schema and behaviour details see [Advanced — Static List](advanced/static-list.md).

---

## `scripts/`

Contains executable scripts used by the `dynamic_list` action type. Scripts can be any executable file — shell scripts, Python programs, compiled binaries, etc. Each script writes its output to stdout and Context Actions parses the result.

File names (without extension) are how commands reference their script:
```yaml
action:
  type: dynamic_list
  config:
    script: team-emails.sh    # runs scripts/team-emails.sh
```

**Output format:**
- **Plain text** — the entire stdout is used as the title of a single result item.
- **JSON array** — an array of `{ "title": "...", "subtext": "..." }` objects (subtext optional).

A seed example (`scripts/hello.sh`) is created automatically on first launch. Context Actions watches this directory and re-runs the active script when any file in `scripts/` changes, so edits take effect immediately.

> **Linux runtime dependency:** Scripts that use `paste_text` as their `item_action` rely on `xdotool` to restore focus to the previous application before pasting. Install it with your package manager (e.g. `sudo apt install xdotool`). Under a pure Wayland session, focus restoration is skipped and the clipboard is set; paste manually with Ctrl+V.

For full details and examples see [Writing Scripts](advanced/writing-scripts.md).

---

- [Configuring Commands](configuring-commands.md) — YAML schema, enable/disable, live reload
- [Basic Functionality](basic/README.md) — Open URL, Paste Text, Copy Text
- [Advanced Features](advanced/README.md) — Static List, Dynamic List, Writing Scripts

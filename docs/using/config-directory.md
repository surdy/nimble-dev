# Config Directory Structure

Ctx stores all per-user data in a single platform-specific config directory. The directory and its subdirectories are created automatically on first launch.

---

## Root directory location

| Platform | Path |
|----------|------|
| macOS | `~/Library/Application Support/com.ctx.launcher/` |
| Linux | `$XDG_CONFIG_HOME/com.ctx.launcher/` (falls back to `~/.config/com.ctx.launcher/`) |
| Windows | `%APPDATA%\com.ctx.launcher\` |

---

## Subdirectory layout

```
com.ctx.launcher/
  commands/         ← YAML command files (watched and hot-reloaded by Ctx)
    examples/       ← seeded on first launch if commands/ is empty
    …               ← your own files and subdirectories
  lists/            ← named list files for the static_list action type
```

Each subdirectory holds a distinct type of data. New subdirectories will be introduced in future releases as new features are added; each will be documented in this file.

---

## `commands/`

Contains all YAML command files. Ctx watches this subdirectory recursively and reloads commands within ~300 ms whenever a file is added, changed, or removed — no restart required.

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

For the full list file schema and behaviour details see [Basic Functionality — List Action](basic-functionality.md#list-action).

---

## Related docs

- [Configuring Commands](configuring-commands.md) — YAML schema, enable/disable, live reload
- [Basic Functionality](basic-functionality.md) — Open URL, Paste Text, Copy Text, Show List action reference

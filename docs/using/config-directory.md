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

## Related docs

- [Configuring Commands](configuring-commands.md) — YAML schema, enable/disable, live reload
- [Basic Functionality](basic-functionality.md) — Open URL, Paste Text, Copy Text action reference

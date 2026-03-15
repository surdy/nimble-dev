# Config Directory Structure

Nimble stores all per-user data in a single platform-specific config directory. The directory and its subdirectories are created automatically on first launch.

---

## Root directory location

| Platform | Path |
|----------|------|
| macOS | `~/Library/Application Support/Nimble/` |
| Linux | `$XDG_CONFIG_HOME/Nimble/` (falls back to `~/.config/Nimble/`) |
| Windows | `%APPDATA%\Nimble\` |

---

## Subdirectory layout

```
Nimble/
  settings.yaml     ← application settings (hotkey, show_context_chip, allow_duplicates)
  commands/         ← YAML command files (watched and hot-reloaded by Nimble)
    examples/       ← seeded on first launch if commands/ is empty
    …               ← your own files and subdirectories
```

All command-related files — YAML configs, list files, and scripts — live within the `commands/` tree. New subdirectories will be introduced in future releases as new features are added; each will be documented in this file.

---

## `settings.yaml`

The `settings.yaml` file at the root of the config directory controls application-level behaviour. It is created automatically the first time you run Nimble.

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

Contains all YAML command files. Nimble watches this subdirectory recursively and reloads commands within ~300 ms whenever a file is added, changed, or removed — no restart required.

You can organise your command files into any subdir structure you like:

```
commands/
  open-github.yaml
  search-google.yaml
  snippets/
    email-signature.yaml
    legal-disclaimer.yaml
  show-team-emails/
    show-team-emails.yaml      ← static_list command
    team-emails.yaml           ← list file, co-located with its command
  say-hello/
    say-hello.yaml             ← dynamic_list command
    hello.sh                   ← script, co-located with its command
  work/
    open-jira.yaml
    paste-standup-template.yaml
```

Commands that use a `static_list` action keep their list file in the same directory as the command YAML. Commands that use `dynamic_list` or `script_action` keep their script in the same directory. See [Advanced — Static List](advanced/static-list.md), [Advanced — Dynamic List](advanced/dynamic-list.md), and [Advanced — Script Action](advanced/script-action.md) for details.

For the full command YAML schema, action types, and live-reload details see [Configuring Commands](configuring-commands.md).

---

- [Configuring Commands](configuring-commands.md) — YAML schema, enable/disable, live reload
- [Basic Functionality](basic/README.md) — Open URL, Paste Text, Copy Text
- [Advanced Features](advanced/README.md) — Static List, Dynamic List, Writing Scripts

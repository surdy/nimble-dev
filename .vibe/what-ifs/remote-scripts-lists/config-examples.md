# Config Examples: Scripts & Lists Anywhere

## Before (current — co-located only)

### Dynamic list

```
commands/search-contacts/
  search-contacts.yaml
  contacts.sh              ← must live here
```

```yaml
# search-contacts.yaml
phrase: search contacts
title: Search contacts
action:
  type: dynamic_list
  config:
    script: contacts.sh      # plain filename, resolved in same directory
    arg: optional
    item_action: paste_text
```

### Static list

```
commands/show-team/
  show-team.yaml
  team-emails.yaml           ← must live here
```

```yaml
# show-team.yaml
phrase: team emails
title: Team email addresses
action:
  type: static_list
  config:
    list: team-emails          # plain name, appends .yaml, same directory
    item_action: paste_text
```

### Shared script problem (today)

Two commands calling the same script must duplicate it:

```
commands/
  copy-uuid/
    copy-uuid.yaml
    uuid.sh               ← copy 1
  paste-uuid/
    paste-uuid.yaml
    uuid.sh               ← copy 2 (identical)
```

---

## After (variable-substituted paths)

### Global env.yaml

```yaml
# ~/Library/Application Support/Nimble/env.yaml
SHARED_SCRIPTS: /Users/me/scripts
SHARED_LISTS: /Users/me/lists
```

### Dynamic list referencing a shared script

```yaml
# commands/search-contacts.yaml  (no subdirectory needed!)
phrase: search contacts
title: Search contacts
action:
  type: dynamic_list
  config:
    script: ${SHARED_SCRIPTS}/contacts.sh    # absolute after substitution
    arg: optional
    item_action: paste_text
```

### Static list referencing a shared list

```yaml
# commands/show-team.yaml
phrase: team emails
title: Team email addresses
action:
  type: static_list
  config:
    list: ${SHARED_LISTS}/team-emails.yaml   # full path with extension
    item_action: paste_text
```

### Shared script — no more duplication

```yaml
# commands/copy-uuid.yaml
phrase: copy uuid
title: Copy a new UUID
action:
  type: script_action
  config:
    script: ${SHARED_SCRIPTS}/uuid.sh
    arg: none
    result_action: copy_text
```

```yaml
# commands/paste-uuid.yaml
phrase: paste uuid
title: Paste a new UUID
action:
  type: script_action
  config:
    script: ${SHARED_SCRIPTS}/uuid.sh   # same script, no duplication
    arg: none
    result_action: paste_text
```

### Using built-in variables

```yaml
# Reference a script relative to config root
phrase: show config
title: List config files
action:
  type: dynamic_list
  config:
    script: ${NIMBLE_CONFIG_DIR}/shared-scripts/list-files.sh
    arg: none
    item_action: copy_text
```

### Co-located default (unchanged)

```yaml
# commands/say-hello/say-hello.yaml — works exactly as before
phrase: say hello
title: Say hello
action:
  type: dynamic_list
  config:
    script: hello.sh          # no ${}, plain name → same directory
    arg: optional
    item_action: paste_text
```

---

## Edge case: undefined variable

```yaml
script: ${UNDEFINED_VAR}/tool.sh
```

Recommended behavior: **fail with a clear error** at invocation time:

```
Error: undefined variable "UNDEFINED_VAR" in script path "${UNDEFINED_VAR}/tool.sh"
```

## Edge case: list field naming

For `static_list`, the `list:` field currently auto-appends `.yaml`. With paths:

| Value | Current behavior | Proposed behavior |
|-------|-----------------|-------------------|
| `team-emails` | `command_dir/team-emails.yaml` | Same (no `${}` → co-located, auto-append) |
| `${LISTS}/team.yaml` | Error (contains `/`) | Load `/path/to/lists/team.yaml` directly |
| `${LISTS}/team` | — | Append `.yaml` → `/path/to/lists/team.yaml` |

The `.yaml` auto-append stays for names without an extension, whether co-located or remote.

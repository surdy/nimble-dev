# Config Examples: Before / After

## Current State (all options)

### Directory layout
```
commands/examples/
├── open-google.yaml              ← command (parsed OK)
├── show-team-emails/
│   ├── show-team-emails.yaml     ← command (parsed OK)
│   └── team-emails.yaml          ← list file (PARSE ERROR in logs)
└── show-user-env/
    ├── show-user-env.yaml        ← command (parsed OK)
    ├── user-env.sh               ← script (ignored — not .yaml)
    └── env.yaml                  ← sidecar env (PARSE ERROR in logs)
```

### Current log output
```
[nimble] could not parse commands/examples/show-team-emails/team-emails.yaml: missing field `phrase`
[nimble] could not parse commands/examples/show-user-env/env.yaml: missing field `phrase`
```

---

## Option A — `.cmd.yaml` suffix for commands

### Directory layout
```
commands/examples/
├── open-google.cmd.yaml              ← command (only .cmd.yaml parsed)
├── show-team-emails/
│   ├── show-team-emails.cmd.yaml     ← command
│   └── team-emails.yaml              ← list file (ignored)
└── show-user-env/
    ├── show-user-env.cmd.yaml        ← command
    ├── user-env.sh
    └── env.yaml                      ← sidecar env (ignored)
```

### Command YAML — no content changes, just filename
```yaml
# show-team-emails.cmd.yaml (was: show-team-emails.yaml)
phrase: team emails
title: Team email addresses
action:
  type: static_list
  config:
    list: team-emails
    item_action: paste_text
```

### List YAML — unchanged
```yaml
# team-emails.yaml — no changes needed
- title: Alice Smith
  subtext: alice@example.com
- title: Bob Jones
  subtext: bob@example.com
```

### Log output
```
(nothing — only .cmd.yaml files are parsed)
```

---

## Option F — Skip `env.yaml` + debug-level logging (recommended)

### Directory layout — NO CHANGES
```
commands/examples/
├── open-google.yaml              ← command (parsed OK)
├── show-team-emails/
│   ├── show-team-emails.yaml     ← command (parsed OK)
│   └── team-emails.yaml          ← list file (skipped, debug log)
└── show-user-env/
    ├── show-user-env.yaml        ← command (parsed OK)
    ├── user-env.sh
    └── env.yaml                  ← sidecar env (skipped by name)
```

### Command YAML — NO CHANGES
```yaml
# show-team-emails.yaml — exactly as before
phrase: team emails
title: Team email addresses
action:
  type: static_list
  config:
    list: team-emails
    item_action: paste_text
```

### List YAML — NO CHANGES
```yaml
# team-emails.yaml — exactly as before
- title: Alice Smith
  subtext: alice@example.com
- title: Bob Jones
  subtext: bob@example.com
```

### Normal log output
```
[nimble] loaded 12 commands (3 files skipped, run with NIMBLE_DEBUG=1 for details)
```

### Debug log output (`NIMBLE_DEBUG=1`)
```
[nimble] skipping commands/examples/show-user-env/env.yaml — reserved filename
[nimble] skipping commands/examples/show-team-emails/team-emails.yaml — not a valid command (missing field `phrase`)
[nimble] loaded 12 commands (2 skipped)
```

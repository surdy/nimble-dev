# Profiles — Config Examples

## Before (current state)

### Config directory

```
Nimble/
  settings.yaml
  env.yaml
  commands/
    open-github.yaml
    open-jira.yaml
    search-google.yaml
    paste-standup.yaml
    open-reddit.yaml
```

### settings.yaml (current)

```yaml
# hotkey: Super+Space
show_context_chip: true
allow_duplicates: true
allow_external_paths: true
```

### Command YAML (current)

```yaml
phrase: open jira
title: Open Jira board
action:
  type: open_url
  config:
    url: https://mycompany.atlassian.net
```

---

## After — Option A (directory-based profiles)

### Config directory

```
Nimble/
  settings.yaml              ← now includes active_profile
  profiles/
    default/                 ← the "no profile" state; always loaded
      env.yaml
      commands/
        search-google.yaml
        open-github.yaml
    work/
      env.yaml               ← JIRA_BASE_URL, WORK_EMAIL, etc.
      commands/
        open-jira.yaml
        paste-standup.yaml
    home/
      env.yaml
      commands/
        open-reddit.yaml
        search-recipes.yaml
```

### settings.yaml

```yaml
# hotkey: Super+Space
show_context_chip: true
allow_duplicates: true
allow_external_paths: true

# Active profile — set via /profile set <name> or edit here
# Omit or set to "default" to load only the default profile commands
active_profile: work
```

### Command YAML (unchanged)

```yaml
# lives in profiles/work/commands/open-jira.yaml
phrase: open jira
title: Open Jira board
action:
  type: open_url
  config:
    url: https://mycompany.atlassian.net
```

No changes to command YAML format. The profile boundary is the directory.

---

## After — Option B (tag-based profiles) — Recommended

### Config directory (unchanged)

```
Nimble/
  settings.yaml              ← now includes active_profile
  env.yaml
  commands/
    open-github.yaml         ← no profiles: field → loaded always
    open-jira.yaml           ← profiles: [work]
    search-google.yaml       ← no profiles: field → loaded always
    paste-standup.yaml       ← profiles: [work]
    open-reddit.yaml         ← profiles: [home]
    search-recipes.yaml      ← profiles: [home]
```

### settings.yaml

```yaml
# hotkey: Super+Space
show_context_chip: true
allow_duplicates: true
allow_external_paths: true

# Active profile — set via /profile set <name> or edit here
# Omit or leave empty to load all commands regardless of profile tags
# active_profile: work
```

### Command YAML — universal (no change)

```yaml
# commands/search-google.yaml — no profiles field, always loaded
phrase: search google
title: Search Google
action:
  type: open_url
  config:
    url: https://google.com/search?q={param}
```

### Command YAML — profile-tagged

```yaml
# commands/open-jira.yaml — only loaded when "work" profile is active (or no profile set)
phrase: open jira
title: Open Jira board
profiles: [work]
action:
  type: open_url
  config:
    url: https://mycompany.atlassian.net
```

### Command YAML — multi-profile

```yaml
# commands/open-slack.yaml — loaded in both work and freelance profiles
phrase: open slack
title: Open Slack
profiles: [work, freelance]
action:
  type: open_url
  config:
    url: https://app.slack.com
```

### Profile-scoped env.yaml (convention)

```yaml
# commands/work-tools/env.yaml — sidecar env for commands in this directory
JIRA_BASE_URL: https://mycompany.atlassian.net
WORK_EMAIL: alice@example.com
```

Global env.yaml remains global. Profile-scoped env is handled through the existing
sidecar env.yaml mechanism — group profile-specific commands in a subdirectory with
their own env.yaml.

---

## Loading behaviour comparison

### No profile active (default)

| Option A | Option B |
|----------|----------|
| Load `profiles/default/commands/` only | Load all commands (tagged and untagged) |

### Profile "work" active

| Option A | Option B |
|----------|----------|
| Load `profiles/default/commands/` + `profiles/work/commands/` | Load untagged commands + commands with `profiles:` containing `work` |

### Profile "home" active

| Option A | Option B |
|----------|----------|
| Load `profiles/default/commands/` + `profiles/home/commands/` | Load untagged commands + commands with `profiles:` containing `home` |

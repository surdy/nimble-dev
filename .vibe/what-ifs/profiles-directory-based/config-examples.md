# Profiles (Directory-Based) — Config Examples

## Current State (before)

### Directory layout

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
    show-team-emails/
      show-team-emails.yaml
      team-emails.yaml
    copy-uuid/
      copy-uuid.yaml
      uuid.sh
```

### settings.yaml

```yaml
# hotkey: Super+Space
show_context_chip: true
allow_duplicates: true
allow_external_paths: true
```

### env.yaml

```yaml
WORK_EMAIL: alice@example.com
JIRA_BASE_URL: https://mycompany.atlassian.net
```

### Example command

```yaml
phrase: open jira
title: Open Jira board
action:
  type: open_url
  config:
    url: https://mycompany.atlassian.net
```

---

## After — Directory-Based Profiles

### Directory layout

```
Nimble/
  settings.yaml                    ← global, unchanged location
  profiles/
    default/                       ← always loaded
      env.yaml                     ← shared env vars (optional)
      commands/
        open-github.yaml
        search-google.yaml
        show-team-emails/
          show-team-emails.yaml
          team-emails.yaml
    work/
      env.yaml                     ← work-specific env vars
      commands/
        open-jira.yaml
        paste-standup.yaml
        copy-uuid/
          copy-uuid.yaml
          uuid.sh
    home/
      env.yaml                     ← home-specific env vars (optional)
      commands/
        open-reddit.yaml
        search-recipes.yaml
```

### settings.yaml (updated)

```yaml
# hotkey: Super+Space
show_context_chip: true
allow_duplicates: true
allow_external_paths: true

# Active profile — commands from profiles/default/ are always loaded.
# Commands from profiles/<active_profile>/ are loaded on top.
# Omit or leave empty to load only profiles/default/.
# active_profile: work
```

### profiles/default/env.yaml (shared env)

```yaml
# Available to all commands in all profiles
GITHUB_USER: alice
```

### profiles/work/env.yaml

```yaml
# Only available when "work" profile is active
WORK_EMAIL: alice@example.com
JIRA_BASE_URL: https://mycompany.atlassian.net
SLACK_WORKSPACE: mycompany
```

### profiles/home/env.yaml

```yaml
# Only available when "home" profile is active
RECIPE_API_KEY: abc123
```

### Command YAML (unchanged format)

```yaml
# profiles/work/commands/open-jira.yaml
# Identical YAML format — no new fields needed
phrase: open jira
title: Open Jira board
action:
  type: open_url
  config:
    url: https://mycompany.atlassian.net
```

### Command with co-located script (unchanged pattern)

```yaml
# profiles/work/commands/copy-uuid/copy-uuid.yaml
phrase: copy uuid
title: Copy a UUID to clipboard
action:
  type: script_action
  config:
    script: uuid.sh
    result_action: copy_text
```

Script `uuid.sh` lives at `profiles/work/commands/copy-uuid/uuid.sh` — same co-location rule as today.

---

## Loading Behaviour

### No active profile (default)

```yaml
# settings.yaml
# active_profile:        ← omitted or empty
```

**Loaded:** `profiles/default/commands/` only  
**Env:** `profiles/default/env.yaml` only  
**Result:** Only universal commands appear

### Profile "work" active

```yaml
# settings.yaml
active_profile: work
```

**Loaded:** `profiles/default/commands/` + `profiles/work/commands/`  
**Env precedence:** `default/env.yaml` → `work/env.yaml` → sidecar → inline  
**Result:** Universal + work commands appear

### Profile "home" active

```yaml
# settings.yaml
active_profile: home
```

**Loaded:** `profiles/default/commands/` + `profiles/home/commands/`  
**Env precedence:** `default/env.yaml` → `home/env.yaml` → sidecar → inline  
**Result:** Universal + home commands appear

---

## Migration Example

### Automatic (Nimble does this on first launch after upgrade)

If `Nimble/commands/` exists and `Nimble/profiles/` doesn't:

1. Create `profiles/default/`
2. Move `commands/` → `profiles/default/commands/`
3. Move `env.yaml` → `profiles/default/env.yaml` (if exists)
4. Log: "Migrated config to profiles/default/"

### Manual

```bash
cd ~/Library/Application\ Support/Nimble
mkdir -p profiles/default
mv commands profiles/default/
mv env.yaml profiles/default/  # if it exists
```

---

## New Built-in Env Variables

| Variable | Value | Example |
|----------|-------|---------|
| `NIMBLE_PROFILE` | Active profile name (empty if none) | `work` |
| `NIMBLE_PROFILE_DIR` | Absolute path to active profile directory | `.../Nimble/profiles/work` |

These supplement the existing `NIMBLE_*` variables. Scripts can use them to
adapt behaviour per profile.

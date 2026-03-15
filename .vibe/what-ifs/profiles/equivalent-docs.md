# Profiles — Equivalent Documentation (Option B)

*This is what the user-facing documentation would look like if tag-based profiles were implemented.*

---

# Profiles

A **profile** controls which commands are loaded into the launcher. Commands tagged with a profile name are only available when that profile is active. Commands without a profile tag are always available.

---

## How it works

Every command YAML file has an optional `profiles` field — a list of profile names:

```yaml
phrase: open jira
title: Open Jira board
profiles: [work]
action:
  type: open_url
  config:
    url: https://mycompany.atlassian.net
```

When the `work` profile is active, this command is loaded. When `home` is active (or no profile is set), it is not.

Commands **without** a `profiles` field are always loaded, regardless of which profile is active:

```yaml
phrase: search google
title: Search Google
action:
  type: open_url
  config:
    url: https://google.com/search?q={param}
```

---

## Managing profiles with built-in commands

| Command | What it does |
|---------|-------------|
| `/profile set <name>` | Activate a profile (e.g. `/profile set work`) |
| `/profile reset` | Clear the active profile — load all commands |
| `/profile show` | Display the currently active profile |

These commands never dismiss the launcher.

### Setting a profile

```
/profile set work
```

Commands are immediately reloaded. Only untagged commands and commands tagged with `work` appear in results.

### Clearing the profile

```
/profile reset
```

All commands are loaded regardless of profile tags.

---

## Tagging commands

### Single profile

```yaml
profiles: [work]
```

### Multiple profiles

```yaml
profiles: [work, freelance]
```

The command is loaded when **any** of the listed profiles is active.

### Universal (no tag)

Omit the `profiles` field entirely. The command is always available.

---

## Profiles vs Contexts

Profiles and contexts serve different purposes and work together:

| | Profile | Context |
|-|---------|---------|
| Controls | Which commands are loaded | How matching works |
| Scope | Entire session | Per-query suffix |
| Example | `/profile set work` → only work commands load | `/ctx set backend` → typing `open` matches `open backend` |

You can combine them: be on the `work` profile with context `backend-team` active.

---

## Settings

The active profile is stored in `settings.yaml`:

```yaml
# Active profile — set via /profile set <name> or edit here
# Omit or leave empty to load all commands regardless of profile tags
# active_profile: work
```

The profile persists across restarts.

---

## YAML schema addition

```yaml
# Optional — list of profile names this command belongs to
# Omitted = always loaded; specified = only loaded when one of these profiles is active
profiles: [<string>, ...]
```

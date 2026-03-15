# What-If: Profiles

## The Question

What if Nimble implemented **named configuration profiles** (e.g. `work`, `home`) — each with their own command set, scripts, and settings — switchable from the launcher or on a schedule?

---

## Key Findings

### What changes for users

Profiles would introduce a top-level organisational concept that sits *above* commands and contexts. Today, all commands live in a single `commands/` tree and the only scoping mechanism is **contexts** (a suffix appended to matching). Profiles would be a harder boundary: switching profiles swaps the entire command set, env variables, and potentially settings.

### Two competing design approaches

#### Option A — Directory-based profiles

Each profile is a subdirectory of the config root containing its own complete config tree:

```
Nimble/
  settings.yaml              ← global (which profile is active, hotkey)
  profiles/
    work/
      settings.yaml          ← profile-specific settings
      env.yaml               ← profile-specific global env
      commands/
        open-jira.yaml
        paste-standup.yaml
    home/
      settings.yaml
      env.yaml
      commands/
        open-reddit.yaml
        search-recipes.yaml
    shared/                  ← special: always loaded regardless of active profile
      commands/
        search-google.yaml
        open-github.yaml
```

**Pros:**
- Complete isolation — each profile is fully self-contained
- Easy to share or back up a profile (zip one folder)
- No risk of command collisions between profiles
- Familiar mental model (like browser profiles)

**Cons:**
- Breaking change — existing `commands/` at root is invalidated
- Duplication — commands you want everywhere must be duplicated or put in `shared/`
- More complex file watcher (watch active profile's tree + shared)
- Migration needed for every existing user

#### Option B — Tag-based profiles (overlay)

Commands stay where they are. Each command YAML gains an optional `profiles:` field. A profile is just a filter applied at load time:

```yaml
# commands/open-jira.yaml
phrase: open jira
title: Open Jira board
profiles: [work]            # only loaded when "work" profile is active
action:
  type: open_url
  config:
    url: https://mycompany.atlassian.net
```

Commands without a `profiles:` field are loaded in every profile (universal). The active profile is stored in `settings.yaml`.

**Pros:**
- Non-breaking — existing commands work as-is (no `profiles:` = always loaded)
- No file reorganisation needed
- Commands can belong to multiple profiles (`profiles: [work, freelance]`)
- Simpler implementation (filter at load time)

**Cons:**
- No isolation — all commands live in the same tree, tagged rather than separated
- Profile-specific env.yaml or settings are harder to model (need naming convention like `env.work.yaml`)
- Harder to share "a profile" as a unit (you'd have to identify all tagged files)
- Cluttered directory for users with many profiles

### Interaction with existing features

| Feature | Impact |
|---------|--------|
| **Contexts** | Profiles and contexts serve different purposes. Contexts are a *matching suffix* (lightweight, ephemeral). Profiles are a *command set filter* (heavyweight, persistent). They coexist: you can be on the `work` profile with context `backend-team`. |
| **Settings** | Profile-specific settings (hotkey? dedup? context chip?) add complexity. Most users would want the same hotkey across profiles but different `allow_duplicates` or `env.yaml`. Needs careful scoping. |
| **env.yaml** | Global env.yaml could be per-profile (Option A naturally supports this; Option B needs `env.work.yaml` convention). |
| **Live reload** | Switching profiles triggers a full reload. The watcher scope changes in Option A (watch different directory). |
| **Copilot agents** | `@nimble-command` needs to know the active profile and tag commands correctly (Option B) or create files in the right profile directory (Option A). |

### Built-in commands needed

```
/profile set work        → switch to the "work" profile
/profile reset           → return to default (no profile / load all)
/profile show            → display current profile name
/profile list            → show all available profiles
```

These follow the same pattern as `/ctx set`, `/ctx reset`, etc.

---

## Opinion

**Mixed — lean positive for Option B (tag-based), skeptical of Option A (directory-based).**

**Why Option B is better for Nimble:**
- Nimble's strength is simplicity. Directory-based profiles (Option A) add a migration burden and break the straightforward "put a YAML file in `commands/`" mental model.
- Tag-based profiles preserve backward compatibility. A user with zero interest in profiles sees no change.
- Most users who want "work" vs "home" commands only have 5-10 commands per context, not hundreds. Tags handle this cleanly.
- Commands that span profiles (`profiles: [work, freelance]`) are a real use case that Option A handles poorly (duplication or symlinks).

**Honest concerns:**
- The overlap with **contexts** is confusing. Contexts *also* scope commands, just via matching rather than filtering. Users will ask: "should I use a context or a profile?" The answer ("contexts narrow matching; profiles control which commands exist") is learnable but not obvious.
- Scheduled profile switching (mentioned in the roadmap) adds significant complexity (cron-like triggers, time zones, what happens mid-typing). I'd recommend deferring scheduling and implementing manual switching first.
- Per-profile settings and env.yaml need a clear convention. If every setting can be per-profile, the settings model becomes complex. I'd recommend only `env.yaml` and `commands/` loading being profile-scoped, with `settings.yaml` remaining global.

**Recommendation:** Implement Option B (tag-based) first. It's backward-compatible, low-risk, and covers the primary use case. If users later demand full isolation, Option A can be layered on top as "workspaces" — a heavier concept for power users.

# What-If: Directory-Based Profiles with Global Settings

## The Question

What if profiles were their own folders — `settings.yaml` stays global at the config root, and all other config (commands, env.yaml, scripts, lists) lives under `profiles/<name>/`?

This is a refinement of the directory-based approach from the [previous profiles analysis](../profiles/summary.md), with a clearer boundary: **settings = global, everything else = per-profile.**

---

## Proposed Config Directory Layout

### Before (current)

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
    say-hello/
      say-hello.yaml
      hello.sh
```

### After

```
Nimble/
  settings.yaml                ← GLOBAL — hotkey, active_profile, show_context_chip, etc.
  profiles/
    default/                   ← always loaded alongside the active profile
      env.yaml                 ← global env for shared commands (optional)
      commands/
        search-google.yaml
        open-github.yaml
        show-team-emails/
          show-team-emails.yaml
          team-emails.yaml
    work/
      env.yaml                 ← work-specific env vars (JIRA_BASE_URL, etc.)
      commands/
        open-jira.yaml
        paste-standup.yaml
        copy-uuid/
          copy-uuid.yaml
          uuid.sh
    home/
      env.yaml
      commands/
        open-reddit.yaml
        search-recipes.yaml
        say-hello/
          say-hello.yaml
          hello.sh
```

### Key rules

1. **`settings.yaml`** stays at the config root — global, never duplicated
2. **`profiles/default/`** is always loaded, regardless of which profile is active — this is where universal commands live
3. **`profiles/<name>/`** is loaded only when that profile is active
4. Active profile is loaded **on top of** `default/` — commands from both are available
5. Each profile directory has its own `commands/` and optional `env.yaml`
6. Co-location rules stay the same — scripts and lists sit next to their command YAML within each profile's `commands/` tree

---

## What Goes Where

| File / Concept | Location | Scope |
|----------------|----------|-------|
| `settings.yaml` | `Nimble/settings.yaml` | Global — all profiles share one settings file |
| `active_profile` | Field in `settings.yaml` | Which profile directory to load alongside `default/` |
| `env.yaml` (global) | `Nimble/profiles/default/env.yaml` | Available to all commands in all profiles |
| `env.yaml` (profile) | `Nimble/profiles/work/env.yaml` | Available to commands in that profile only |
| `env.yaml` (sidecar) | Next to a command YAML | That command only (unchanged from today) |
| Commands | `Nimble/profiles/<name>/commands/` | Loaded when that profile is active |
| Scripts / Lists | Co-located with their command YAML | Unchanged — same directory as command |

### Env precedence (updated)

```
System environment
NIMBLE_* built-ins
profiles/default/env.yaml        ← new layer
profiles/<active>/env.yaml       ← new layer
command-dir sidecar env.yaml
command inline env:
```

---

## Settings.yaml Changes

```yaml
# Nimble/settings.yaml — GLOBAL

# hotkey: Super+Space
show_context_chip: true
allow_duplicates: true
allow_external_paths: true

# Active profile — set via /profile set <name> or edit here.
# Commands from profiles/default/ are always loaded.
# Commands from profiles/<active_profile>/ are loaded on top.
# Omit or leave empty to load only profiles/default/.
# active_profile: work
```

Settings that stay global (same regardless of profile):
- `hotkey`
- `show_context_chip`
- `allow_duplicates`
- `allow_external_paths`
- `active_profile`

Nothing per-profile in settings. If a user wants different `allow_duplicates` per profile, that's not supported — settings are always global. This keeps the model simple.

---

## Built-in Commands

```
/profile set work       → activate "work" profile, reload commands
/profile reset          → clear profile, load only default/
/profile show           → display current profile name
/profile list           → list all profile directories found in profiles/
```

`/profile list` scans `profiles/` for subdirectories (excluding `default/`) and shows them as results.

---

## What Changes From the User's Perspective

### First-time users (no existing config)

On first launch, Nimble seeds:

```
Nimble/
  settings.yaml
  profiles/
    default/
      commands/
        examples/
          open-github.yaml
          search-google.yaml
          say-hello/
            say-hello.yaml
            hello.sh
```

The experience is **nearly identical** to today. Commands are in `profiles/default/commands/` instead of `commands/`. The user never thinks about profiles unless they want to.

### Existing users (migration required)

This is a **breaking change**. Existing users must move their files:

```bash
# macOS migration
cd ~/Library/Application\ Support/Nimble
mkdir -p profiles/default
mv commands profiles/default/
mv env.yaml profiles/default/   # if it exists
```

Or Nimble could auto-migrate on first launch: if `commands/` exists at the config root and `profiles/` doesn't, move `commands/` and `env.yaml` into `profiles/default/` automatically.

### Creating a new profile

1. Create a directory: `mkdir -p profiles/work/commands/`
2. Add command YAMLs inside it
3. Switch to it: `/profile set work`

Or from the launcher: `/profile set work` could create the directory if it doesn't exist (with a confirmation).

### Daily workflow

```
1. Open launcher
2. /profile set work              ← commands reload, work + default loaded
3. Type commands as usual          ← results come from both pools
4. /profile set home              ← swap to home commands
5. /profile reset                 ← back to default only
```

The profile persists across restarts (stored in `settings.yaml`). Most users would set it once and rarely change it.

### Sharing a profile

Zip `profiles/work/` and send it. The recipient drops it into their `profiles/` directory. Self-contained — commands, scripts, lists, and env.yaml all travel together.

---

## Impact on Existing Features

### Contexts

Unchanged. Contexts operate at the matching layer, profiles operate at the loading layer. They compose:

- Profile `work` active → only `default/` + `work/` commands loaded
- Context `backend` active → matching appends `backend` to typed input
- Both work together, no conflict

### Live reload / Watcher

The watcher scope changes:

**Before:** Watch `commands/` recursively.

**After:** Watch `profiles/default/commands/` + `profiles/<active>/commands/` recursively. On profile switch, stop watching the old profile directory and start watching the new one.

Env.yaml changes in `profiles/default/` or `profiles/<active>/` also trigger reloads.

### `NIMBLE_CONFIG_DIR` and `NIMBLE_COMMAND_DIR`

- `NIMBLE_CONFIG_DIR` stays the config root (`Nimble/`) — unchanged
- `NIMBLE_COMMAND_DIR` now points into `profiles/<name>/commands/<subdir>/` — the absolute path to the command YAML's directory, same semantic as today
- New built-in: **`NIMBLE_PROFILE`** — the active profile name (empty string if only default)

### `${VAR}` substitution

`NIMBLE_CONFIG_DIR` still works for referencing shared resources. A new `NIMBLE_PROFILE_DIR` could point to the active profile's root:

```yaml
script: ${NIMBLE_PROFILE_DIR}/shared-scripts/helper.sh
```

### `source_dir` resolution

Currently `source_dir` is relative to `config_dir/commands/`. With profiles, it becomes relative to `config_dir/profiles/<name>/commands/`. The backend resolves this — the frontend still passes `source_dir` and the backend knows where to look.

### Copilot agents

`@nimble-command` needs to know the active profile to place files in the right directory. It would ask "which profile should this command go in?" or default to `default/`.

---

## Edge Cases

### 1. Duplicate phrases across default + active profile

Two files define `phrase: open github` — one in `default/`, one in `work/`.

**Recommended:** Active profile wins. Commands from the active profile override same-phrase commands from `default/`. This lets you have a "generic" version in default and a "work-specific" override in the work profile. Controlled by `allow_duplicates` setting (if `false`, a warning is shown).

### 2. User sets a profile that doesn't exist

`/profile set gaming` but `profiles/gaming/` doesn't exist.

**Recommended:** Show an error message in the results list: "Profile 'gaming' not found. Create it at profiles/gaming/commands/". Don't silently fall back to default — that would be confusing.

Alternatively, create the directory automatically so it's ready for the user to populate.

### 3. Empty profile

`profiles/work/` exists but `profiles/work/commands/` is empty or missing.

**Recommended:** Load default commands only. No error — an empty profile just means "default commands plus nothing extra."

### 4. Nested profiles

`profiles/work/profiles/deep/` — should this be treated as a sub-profile?

**Recommended:** No. Profile discovery is one level deep only. Subdirectories within a profile's `commands/` tree are for organising commands, not for nesting profiles.

### 5. Profile-specific env.yaml precedence

Does `profiles/work/env.yaml` override `profiles/default/env.yaml`?

**Recommended:** Yes. Precedence: `default/env.yaml` → `<active>/env.yaml` → sidecar → inline. The active profile's env overrides the default profile's env, just as sidecar overrides global today.

---

## Comparison With Tag-Based Approach

| Aspect | Directory-based (this proposal) | Tag-based (previous analysis) |
|--------|------|------|
| **Breaking change** | Yes — migration required | No — fully backward-compatible |
| **Shareability** | Excellent — zip a folder | Poor — grep tagged files |
| **Multi-profile commands** | Duplicate or put in `default/` | `profiles: [work, home]` on one file |
| **Isolation** | Complete — separate file trees | None — all files in one tree |
| **Per-profile env.yaml** | Natural — each profile has its own | Awkward — needs naming convention |
| **Complexity** | Higher — watcher, path resolution, migration | Lower — filter at load time |
| **Mental model** | "Browser profiles" — familiar | "Tags" — less common but flexible |
| **First-run experience** | Slightly more nested (`profiles/default/commands/`) | Unchanged (`commands/`) |

---

## Opinion

**More positive than before on directory-based, given the global-settings constraint.**

The previous analysis flagged directory-based profiles as heavyweight. But with `settings.yaml` staying global, the main objection (per-profile settings complexity) disappears. What remains is clean:

**In favour:**
- **Shareability is the killer feature.** Zip `profiles/work/`, send it to a colleague, done. This is the single biggest advantage over tag-based. For teams using Nimble, it's a real workflow.
- **Clean isolation.** No accidental command collisions between profiles. No "forgot to tag this file" mistakes.
- **Per-profile env.yaml is natural.** `JIRA_BASE_URL` in `profiles/work/env.yaml` makes intuitive sense. No naming conventions needed.
- **The `default/` profile as "always loaded" is elegant.** Commands you always want go in one place, profile-specific commands go in another. Clear separation.

**Concerns:**
- **Breaking change.** Every existing user must reorganise. Auto-migration can mitigate this, but it's still friction.
- **Slightly deeper nesting** for the simplest case. New users create commands in `profiles/default/commands/` instead of `commands/`. Three path components deeper. Not terrible, but not as inviting.
- **Multi-profile commands require `default/`.** A command you want in both `work` and `home` but NOT in other hypothetical profiles can't be expressed without duplication. Putting it in `default/` means it loads in *all* profiles, not just work + home. This is a real limitation vs tag-based.
- **Watcher complexity.** Watching two directory trees and swapping on profile change is more work than filtering a flat list. Not a dealbreaker, but more code to maintain.

**Overall:** If shareability and isolation are priorities, this is the right design. If backward-compatibility and zero-migration are priorities, tag-based still wins. For a power-user-oriented tool like Nimble, I'd say directory-based with auto-migration is a solid choice — the nesting is a small price for clean separation.

**Recommendation:** If you go this route, implement auto-migration (detect old `commands/` at root, move to `profiles/default/`) and make the first-run seeding use `profiles/default/commands/examples/`. The breaking change becomes transparent for most users.

# Profiles (Directory-Based) — UX Impact

## Setup Experience

### New users

First launch creates:

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

The user's first command goes in `profiles/default/commands/`. The path is slightly longer than today's `commands/`, but the structure is predictable. The user only encounters profiles when they actively want them.

**Impact:** Minimal. One extra folder level (`profiles/default/`) in the path. The Getting Started docs would reference `profiles/default/commands/` instead of `commands/`.

### Existing users

Breaking change. On first launch after upgrade, Nimble auto-migrates:

1. Detects `commands/` at root with no `profiles/` directory
2. Creates `profiles/default/`
3. Moves `commands/` → `profiles/default/commands/`
4. Moves `env.yaml` → `profiles/default/env.yaml` (if present)
5. Logs the migration path

**Impact:** Transparent if auto-migration works. Users with scripts referencing `NIMBLE_CONFIG_DIR` + relative paths may need adjustments since the directory tree is deeper. `NIMBLE_COMMAND_DIR` (absolute path to the command's directory) still works — it just points to the new location.

---

## Creating a Profile

### Minimal (just commands)

```bash
mkdir -p ~/Library/Application\ Support/Nimble/profiles/work/commands
```

Then add command YAML files to `profiles/work/commands/`.

### With env vars

```bash
mkdir -p ~/Library/Application\ Support/Nimble/profiles/work/commands
cat > ~/Library/Application\ Support/Nimble/profiles/work/env.yaml << 'EOF'
JIRA_BASE_URL: https://mycompany.atlassian.net
WORK_EMAIL: alice@example.com
EOF
```

### From someone else's shared profile

```bash
cd ~/Library/Application\ Support/Nimble/profiles
unzip ~/Downloads/work-profile.zip
# creates profiles/work/ with commands/ and env.yaml inside
```

---

## Switching Profiles

### From the launcher

```
/profile set work        → loads default + work commands
/profile set home        → loads default + home commands
/profile reset           → loads default only
/profile show            → shows "work" (or "No profile active")
/profile list            → shows all available profiles (scans profiles/ for dirs)
```

All `/profile` commands keep the launcher open (same UX as `/ctx` commands).

### What happens on switch

1. `active_profile` in `settings.yaml` is updated
2. File watcher stops watching old profile's `commands/`
3. File watcher starts watching new profile's `commands/`
4. All commands are reloaded: `default/commands/` + `<new profile>/commands/`
5. Launcher results update immediately
6. Context (if active) is preserved — profiles and contexts are independent

### Visual indicator

A **profile chip** in the launcher bar (similar to the context chip) showing the active profile name. Could be a different colour to distinguish from the context chip. Settings control: `show_profile_chip: true`.

```
┌──────────────────────────────────────────────────────────────┐
│  open ji...                                    work │ reddit │
│                                              [profile] [ctx] │
└──────────────────────────────────────────────────────────────┘
```

---

## Daily Workflow

### Typical day

1. Nimble launches with `active_profile: work` (persisted from yesterday)
2. Type commands as usual — work + default commands are available
3. End of workday: `/profile set home` or just leave it on work
4. Home commands appear, work commands disappear

Most users would switch profiles 0–2 times per day. Some would never switch — they'd put everything in `default/` and ignore profiles entirely.

### Sharing with a colleague

```bash
# Export
cd ~/Library/Application\ Support/Nimble/profiles
zip -r work-profile.zip work/

# Colleague imports
cd ~/Library/Application\ Support/Nimble/profiles
unzip ~/Downloads/work-profile.zip
# Done — /profile set work to activate
```

Self-contained: commands, scripts, lists, and env.yaml all travel in one zip.

---

## Overlap With Contexts

| | Profiles | Contexts |
|-|----------|----------|
| **Metaphor** | "Which desk am I sitting at?" | "What topic am I focused on?" |
| **What it controls** | Which commands exist | How matching narrows results |
| **Switch frequency** | Once a day (or never) | Multiple times per session |
| **Persistence** | `settings.yaml` | `localStorage` |
| **Built-in commands** | `/profile set/reset/show/list` | `/ctx set/reset` |
| **Visual indicator** | Profile chip (left? separate colour?) | Context chip (right, blue) |

They compose cleanly. Example:
- Profile `work` → loads Jira, Slack, standup commands
- Context `backend` → narrows `open` to match `open backend-dashboard`
- Both active simultaneously, no conflict

---

## Potential Friction Points

1. **Deeper path for simple case.** New users create commands in `profiles/default/commands/my-command.yaml` instead of `commands/my-command.yaml`. The extra nesting feels heavier for someone who just wants one command.

2. **"Where did my command go?"** After switching profiles, a command from the previous profile disappears. The user might not remember they switched profiles. Mitigation: always-visible profile chip.

3. **Commands in the wrong profile.** User creates a command directly inside `profiles/work/` instead of `profiles/work/commands/`. Nimble doesn't find it. Mitigation: clear error messages, good docs.

4. **Cross-profile commands.** "I want this command in work AND home but NOT in other profiles." Only option: put a copy in both `profiles/work/commands/` and `profiles/home/commands/`. Duplication. The tag-based approach handles this more cleanly.

5. **Two launcher chips.** Profile chip + context chip could feel cluttered, especially on narrow screens. Consider showing only one at a time, or combining them: `work · reddit`.

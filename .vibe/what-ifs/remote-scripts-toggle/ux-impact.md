# UX Impact: Remote Scripts with Config Toggle

## What changes for users

### The default experience (allow_external_paths: true)

**Nothing changes for co-located commands.** Plain filenames keep working.

**Shared scripts just work.** Set a variable, reference it, done:

```yaml
# env.yaml
SHARED_SCRIPTS: /Users/me/scripts

# any command YAML
script: ${SHARED_SCRIPTS}/uuid.sh   # resolves, runs, no extra config needed
```

No setting to flip. No "unlock" step. It works out of the box.

### The restricted experience (allow_external_paths: false)

A security-conscious user adds one line to `settings.yaml`:

```yaml
allow_external_paths: false
```

From that point:
- Co-located scripts and lists: **work normally**
- `${VAR}/script.sh` resolving outside command dir: **blocked with clear error**
- `${NIMBLE_COMMAND_DIR}/script.sh` (resolves inside command dir): **still works**

The error tells them exactly why and how to change it:

```
Script path "/Users/me/scripts/uuid.sh" resolves outside the command
directory. Set allow_external_paths: true in settings.yaml to allow
external script paths.
```

### Who sets allow_external_paths: false?

| User type | Would they set false? | Why? |
|-----------|----------------------|------|
| Solo power user | No | Wants max flexibility |
| Beginner | No | Never encounters external paths |
| Corporate admin pre-seeding configs | Yes | Policy enforcement |
| Shared machine user | Maybe | Wants to audit what runs |
| Security-conscious minimalist | Maybe | Principle of least authority |

Most users never touch this setting. It exists for the small minority who actively want containment.

### Discoverability

The setting is documented in:
1. `settings.yaml` (with a comment explaining what it does)
2. `docs/using/config-directory.md` (in the settings section)
3. `docs/using/advanced/writing-scripts.md` (in the remote paths section)
4. The **error message itself** (when a user hits the restriction without knowing it)

Point 4 is the most important — if someone copies a command YAML from a shared config that uses `${VAR}` paths, and they have `false` set, the error immediately explains the fix.

### Interaction with other settings

| Setting | Interaction |
|---------|-------------|
| `allow_duplicates` | Independent — no interaction |
| `show_context_chip` | Independent — no interaction |
| `hotkey` | Independent — no interaction |

The setting does **not** affect:
- `env.yaml` loading (global and sidecar always loaded)
- `NIMBLE_*` built-in variables (always injected)
- Script argument passing, timeout, output parsing
- Static list item actions

## What doesn't change

Everything from the base "remote scripts" what-if holds. The toggle is purely additive — it doesn't change the feature, just adds a guardrail.

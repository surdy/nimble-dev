# Config Impact and Path Strategy

## 2) Exact path strategy across macOS/Linux/Windows

### Proposed variable semantics

- `NIMBLE_COMMAND_CACHE_DIR`: volatile per-command directory for disposable artifacts.
- `NIMBLE_COMMAND_DATA_DIR`: persistent per-command directory for state to keep across restarts.

### Cross-platform roots

- **Cache root**: platform cache dir + `Nimble/commands/`
- **Data root**: platform app data dir + `Nimble/commands/`

Suggested concrete roots:

- **macOS**
  - Cache: `~/Library/Caches/Nimble/commands/`
  - Data: `~/Library/Application Support/Nimble/commands/`
- **Linux**
  - Cache: `$XDG_CACHE_HOME/Nimble/commands/` (fallback `~/.cache/Nimble/commands/`)
  - Data: `$XDG_DATA_HOME/Nimble/commands/` (fallback `~/.local/share/Nimble/commands/`)
- **Windows**
  - Cache: `%LOCALAPPDATA%/Nimble/Cache/commands/`
  - Data: `%APPDATA%/Nimble/commands/`

### Naming/sanitization for deterministic per-command folders

Use a stable identifier derived from command file location, not phrase/title:

- Input identity: `source_dir + command_yaml_filename`
- Normalize to UTF-8 bytes and hash (e.g., SHA-256), then truncate (e.g., first 16 hex chars)
- Directory name format:
  - `cmd-<hash>`

Optional human hint (non-authoritative):
- `cmd-<hash>__<slug>` where slug is sanitized and truncated; hash remains source of truth.

### Behavior on command phrase/title changes

- Phrase/title edits **must not** change storage path.
- Storage identity is tied to command file path identity, so cosmetic text edits are safe.
- If the command YAML file is moved/renamed, identity changes and a new storage directory is used.

This is predictable and avoids hidden coupling to user-facing labels.

---

## 3) Scope model

### Recommended scope

- **Per-command** scope (primary)
- Available to **both** `dynamic_list` and `script_action`

Why:
- Both actions execute scripts and need identical environment semantics.
- Per-directory would create accidental coupling between unrelated commands.
- Per-action scope is too coarse and does not map to individual script state.

### Optional future extension

Allow explicit shared storage groups later (e.g., `storage_group`) if users need multiple commands to share durable state intentionally.

---

## 7) Backward compatibility and migration impact

- Additive and non-breaking: existing scripts keep working without modification.
- Scripts can opt into new vars gradually.
- No migration required for existing command YAML.
- If command files are moved, storage paths change by design (documented behavior).

---

## Config examples

### No YAML schema change required (preferred)

These are built-ins; users do not configure them.

`script_action` example:

```yaml
phrase: sync contacts
title: Sync contacts cache
action:
  type: script_action
  config:
    script: sync-contacts.sh
    arg: none
    result_action: copy_text
```

Script behavior (conceptually):
- Store temporary API responses in `NIMBLE_COMMAND_CACHE_DIR`
- Store durable sync checkpoint in `NIMBLE_COMMAND_DATA_DIR`

### Optional visibility in docs only

No command fields added; docs should include examples showing when to write to cache vs data and naming conventions for files under each directory.

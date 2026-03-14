# UX Impact: Data Directory for script_action

## What Changes from the User's Perspective

### Writing a new script command that reads external data

| Step | Current | With `data_dir` |
|------|---------|-----------------|
| 1 | Write script, hard-code path to external data | Write script, read `$NIMBLE_DATA_DIR` env var |
| 2 | Create command YAML | Create command YAML with `data_dir: ~/path/to/data` |
| 3 | To change the path: edit the script | To change the path: edit the YAML — script stays the same |

**Net effect**: Data location is configuration, not code. The YAML becomes the single place to configure a command.

### Sharing a script command

| Scenario | Current | With `data_dir` |
|----------|---------|-----------------|
| Share with a colleague | "Edit line 3 of the script to change the path" | "Set `data_dir` in the YAML to wherever your data lives" |
| Publish to a community repo | Script has your personal path in it — must parameterize manually | Script is already parameterized — YAML is the only thing to customize |

### Day-to-day usage

| Scenario | Current | With `data_dir` |
|----------|---------|-----------------|
| External app changes export location | Edit the script | Edit one line in the YAML |
| Same script, two commands, different data dirs | Duplicate the script or add logic to accept a path arg | Two YAMLs with different `data_dir` values, same script |
| Script doesn't need external data | (no change) | Omit `data_dir` — everything works as before |

### The "same script, different data" pattern

This is a powerful new capability. One generic script can serve multiple commands:

```yaml
# commands/search-work-contacts.yaml
phrase: search work contacts
title: Search work contacts
action:
  type: dynamic_list
  config:
    script: search-json.sh         # same generic script
    arg: optional
    item_action: paste_text
    data_dir: ~/Data/work-crm

# commands/search-personal-contacts.yaml
phrase: search personal contacts
title: Search personal contacts
action:
  type: dynamic_list
  config:
    script: search-json.sh         # same script, different data
    arg: optional
    item_action: paste_text
    data_dir: ~/Data/personal-crm
```

`scripts/search-json.sh` doesn't care where the data is — it reads `$NIMBLE_DATA_DIR/*.json`.

---

## Error Handling

### What the user sees when `data_dir` is invalid

| Situation | Behavior |
|-----------|----------|
| `data_dir` doesn't exist | Command rejected at load time with a warning (same as a missing `list:` file today) |
| `data_dir` is outside `$HOME` | Command rejected at load time: "data_dir must be within your home directory" |
| `data_dir` points to a file, not a directory | Command rejected at load time: "data_dir must be a directory" |
| `data_dir` exists but script fails | Normal script error handling (5s timeout, stderr logged) |

### Load-time vs run-time validation

**Option A — Validate at load time** (recommended):
- Command is rejected when the YAML is loaded if the directory doesn't exist
- User gets immediate feedback when saving the file
- Downside: if the external app creates the directory later, user must reload

**Option B — Validate at run time**:
- Command loads fine; error only when script is invoked
- More forgiving for directories that come and go
- Downside: user doesn't know about typos until they try to use the command

**Recommendation**: Validate at load time — consistent with how `list:` file references work today. If the directory is missing, the user sees a warning immediately after saving.

---

## Security Considerations

| Concern | Mitigation |
|---------|-----------|
| Point to system directories (`/etc/`) | Reject paths not within `$HOME` |
| Path traversal (`~/../../etc/`) | Canonicalize path, then re-check it starts with `$HOME` |
| Symlink escape (`~/data` → `/etc/`) | Resolve symlinks before checking containment |
| Script writes to the directory | Not Nimble's concern — scripts already run with user privileges. `data_dir` is read-only by convention, not enforcement |

The `$HOME`-only restriction is pragmatic, not airtight — a user could symlink a directory inside `$HOME` to anywhere. But it prevents the most common mistakes and makes the intent clear: `data_dir` is for *your* data in *your* home directory.

---

## What Doesn't Change

- Scripts still live in `scripts/`
- Scripts still write to stdout, stderr is logged
- 5-second timeout unchanged
- `arg` modes unchanged
- All existing command YAMLs work without modification (field is optional)
- The `scripts/` security boundary (plain filename, no path separators) is unchanged

---

## Naming Alternatives Considered

| Name | Pros | Cons |
|------|------|------|
| `data_dir` | Clear intent, matches the concept | Slightly generic |
| `input_dir` | Emphasizes "input to the script" | Could be confused with user input |
| `working_dir` | Familiar term | Implies `cwd`, which is a different concept |
| `source_dir` | Clear direction (data flows from here) | Could be confused with source code |
| `watch_dir` | Suggests directory monitoring | Misleading — Nimble doesn't watch it |

**`data_dir`** is the best fit — it's descriptive, unambiguous, and doesn't collide with existing concepts.

---

## Verdict

| Factor | Assessment |
|--------|-----------|
| Solves a real problem? | Yes — external data integration is a concrete need |
| Additive / non-breaking? | Yes — purely optional field |
| Simple to understand? | Yes — "where the data lives" |
| Consistent with Nimble design? | Yes — one field, one purpose, validated at boundaries |
| Security risk? | Low — restricted to `$HOME`, validated at load time |
| Overall | **Positive** — small, focused addition that enables meaningful new workflows |

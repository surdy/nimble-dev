# UX Impact: Filename Convention for Command YAML

## What changes for users?

### Option A (`.cmd.yaml` suffix) — Breaking change

1. **Every command file must be renamed.** Users with existing configs must rename all `.yaml` files to `.cmd.yaml`. A user with 20 commands needs 20 renames.

2. **Documentation and examples change.** Every doc page, example config, seed file, and agent instruction that references `.yaml` must be updated. The phrase "create a YAML file" becomes "create a `.cmd.yaml` file".

3. **Mental model changes.** Users need to remember a new convention. "Why `.cmd.yaml` and not just `.yaml`?" is a question every new user will ask. The answer ("because list files are also YAML") is unsatisfying — it exposes an implementation detail.

4. **Agent definitions need updating.** Both `@nimble-command` and `@nimble-script` need to know about the new extension. The spec file needs a `command_file_extension` field.

5. **File watcher needs updating.** `is_yaml_event()` in `watcher.rs` would need to filter for `.cmd.yaml` specifically (or still watch all YAML and only reload commands for `.cmd.yaml` changes).

6. **Migration path needed.** Existing users need a migration guide or script. The app could auto-detect old-style `.yaml` files and log a deprecation warning, but this adds complexity.

### Option F (debug-level logging) — No user-facing change

1. **Nothing changes for users.** Existing configs, file naming, documentation, agents, and mental models all remain identical.

2. **Log noise disappears.** Users who previously noticed parse errors for list/env files will see a clean summary line instead.

3. **Debugging is slightly different.** Users with a genuinely broken command file won't see the error by default — they need `NIMBLE_DEBUG=1`. However, the summary line ("3 files skipped") serves as a signal that something was ignored.

4. **Stage 30 alignment.** Stage 30 (Script debugging and verbose logs) already plans to add `NIMBLE_DEBUG` — this change slots naturally into that work.

## Daily workflow comparison

| Scenario | Option A | Option F |
|----------|----------|----------|
| Create a new simple command | Create `my-command.cmd.yaml` | Create `my-command.yaml` |
| Create a static_list command | Create `my-list.cmd.yaml` + `data.yaml` | Create `my-list.yaml` + `data.yaml` |
| Rename/move a command | Rename `*.cmd.yaml` | Rename `*.yaml` |
| Debug a broken command | Check logs (error visible) | Run with `NIMBLE_DEBUG=1`, check logs |
| First-time user creates a file | Must know about `.cmd.yaml` convention | Just create any `.yaml` file |

## Recommendation

Option F is the clear winner for daily workflow. The `.cmd.yaml` convention adds friction to the most common operation (creating a command) to solve a problem that only surfaces in logs. The debug-level logging approach eliminates the noise without imposing any new conventions on users.

# UX Impact: Scripts & Lists Anywhere

## What changes for users

### Daily workflow

**No change for simple setups.** Users who keep scripts co-located never see anything different. Plain filenames work exactly as they do today.

**Power users gain shared scripts.** Instead of copying `uuid.sh` into three command directories, set a `SHARED_SCRIPTS` variable in `env.yaml` and reference it from any command. Edit the script once, all commands use the update.

**Teams can share a script repo.** Clone a team-maintained scripts repo to a fixed path, point `SHARED_SCRIPTS` at it, and commands across the org reference the same tools without any Nimble-specific packaging.

### Setup complexity

| Scenario | Before | After |
|----------|--------|-------|
| Simple co-located script | 1 dir + 1 YAML + 1 script | Same |
| Script shared by 2 commands | 2 dirs + 2 YAMLs + 2 script copies | 2 YAMLs + 1 script + 1 env var |
| Script shared by 5 commands | 5 dirs + 5 YAMLs + 5 copies | 5 YAMLs + 1 script + 1 env var |
| External script repo | Not supported | 1 env var, works |

### Error experience

Today when a script fails, the error says:
```
Script not found: /Users/me/Library/Application Support/Nimble/commands/foo/bar.sh
```

After this change, errors with variables should show the **resolved path**:
```
Script not found: /Users/me/scripts/bar.sh (resolved from ${SHARED_SCRIPTS}/bar.sh)
```

Showing both the original template and the resolved path is essential for debuggability.

### Mental model shift

**Before:** Scripts are always right next to their command YAML. Finding a script = finding the command directory.

**After:** Scripts are *usually* co-located, but *could be* anywhere. When debugging, you need to check whether the `script:` field uses variables. The `env.yaml` files become part of the debugging chain.

This is a manageable complexity increase for the type of user who would reach for this feature. Beginners never encounter it.

### Security perception

The current docs say scripts are "co-located" and can't escape the command directory. After this change:
- Scripts can be anywhere the user configures.
- The user is still the one writing the YAML and env files — no external actor can change script paths.
- This aligns with how Alfred, Raycast, etc. work (scripts can be anywhere).
- The security model shifts from "filesystem containment" to "user intent" — which is arguably more honest about what Nimble can actually enforce.

### Config directory impact

No new directories or files required. Uses the existing `env.yaml` system from Stage 29.

Optional: a `shared/` or `scripts/` directory at the config root could be a recommended convention (not enforced), documented as a best practice for shared scripts.

## What doesn't change

- `arg` modes (none / optional / required)
- Script output format (JSON array or plain text)
- 5-second timeout
- Built-in `NIMBLE_*` environment variables
- List item actions (paste_text, copy_text, open_url)
- sidecar and inline `env:` — still work, still follow the same precedence
- Co-located scripts and lists — still the default, still recommended for simple setups

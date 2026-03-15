# What If: Allow scripts and lists to live anywhere

_Date: 2026-03-15_

## The idea

Today, `script:` and `list:` fields only accept plain filenames co-located with the command YAML. The proposal: let users point these fields at files **anywhere** on disk by leveraging variable substitution in the path. The co-located default remains unchanged — this is purely opt-in.

### How it would work

The `script:` and `list:` fields would support a simple `${VAR}` syntax, resolved against the same env layers scripts already receive (global `env.yaml`, sidecar `env.yaml`, inline `env:`, plus built-in `NIMBLE_*` variables).

If the resolved path is absolute, use it directly. If it's a plain filename (no separators), resolve relative to the command directory (current behavior). If it's a relative path with separators, resolve relative to the command directory.

## Key findings

### Positives

1. **Shared scripts become natural.** Today, two commands that call the same script must each have a copy. With this change, you define `SCRIPTS_DIR: /path/to/scripts` in global `env.yaml` and reference `script: ${SCRIPTS_DIR}/shared-tool.sh` from any command.

2. **Leverages existing infrastructure.** We just shipped three-layer user-defined variables (Stage 29). This reuses that same system — no new config surface to learn.

3. **Zero breaking changes.** A plain filename like `hello.sh` contains no `${}` tokens, so existing commands keep working identically. The path traversal/separator rejection only kicks in for literal `/` and `..` — the substituted path would bypass it intentionally.

4. **Helpful for monorepo-style setups.** Teams that keep scripts in a shared repo directory outside the Nimble config tree get first-class support.

### Concerns

1. **Security boundary erosion.** Today, scripts can only run files inside the command directory. Allowing arbitrary paths means a command YAML could point at `/usr/bin/rm` or any executable on disk. This is a meaningful weakening of the sandbox. Mitigations:
   - The **user** writes the YAML and the env vars — this isn't untrusted input.
   - Nimble already runs scripts with the user's full privileges.
   - But it shifts the security posture from "scripts are contained" to "scripts can be anywhere the user points to."

2. **Variable substitution complexity.** Even a simple `${VAR}` syntax introduces edge cases: undefined variables, nested references, escaping literal `${}`. Need to decide: fail on undefined var? Leave the token as-is? Use empty string?

3. **Discoverability problem for lists.** For `static_list`, the `list:` field currently means "look for `<name>.yaml` in this directory." If it becomes a path, do we keep the `.yaml` auto-append? Or require the full filename? Mixing conventions feels messy.

4. **Debugging gets harder.** When a script fails, the user now needs to mentally resolve variables to figure out which file ran. Error messages should show the resolved path.

5. **Documentation overhead.** Three separate docs (static-list, dynamic-list, script-action) all need updated schema sections, examples, and caveats.

## Opinion

**Mixed — lean slightly positive, but timing may be premature.**

The feature solves a real pain point (shared scripts across commands) and elegantly reuses the env variable system we just built. The co-located default stays simple for beginners.

However, the security boundary change is non-trivial. The current design document explicitly states that scripts are "sandboxed" to their command directory. This change weakens that promise. Since the user controls both the YAML and the env files, the practical risk is low — but the *mental model* shifts.

A simpler alternative worth considering first: a `scripts/` or `shared/` directory at the config root that commands can reference with a `shared:` prefix (e.g., `script: shared:my-tool.sh`). This gives reusability without arbitrary filesystem access. But it's less flexible and adds a new concept.

**Recommendation:** If you want this, implement it — but be deliberate about: (a) failing loudly on undefined variables, (b) showing resolved paths in errors, and (c) documenting the security posture change. The variable substitution should be limited to `${VAR}` only (no nested expansion, no shell-style `$VAR`).

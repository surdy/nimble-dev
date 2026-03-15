# What If: Remote Scripts Enabled by Default + Config Toggle

_Date: 2026-03-15_
_Builds on: [remote-scripts-lists](../remote-scripts-lists/summary.md)_

## The variation

Same as the previous what-if (allow `${VAR}` in `script:` and `list:` paths), but with a twist: the capability is **enabled by default** and security-conscious users can **opt out** via a `settings.yaml` toggle.

```yaml
# settings.yaml
allow_external_paths: false   # locks scripts/lists to co-located only
```

When `false`, any `script:` or `list:` value that resolves to a path outside the command directory is rejected at invocation time with a clear error.

## Analysis

### How the setting works

| `allow_external_paths` | `script: hello.sh` | `script: ${SHARED}/tool.sh` |
|---|---|---|
| `true` (default) | Resolves in command dir | Resolves to external path, runs it |
| `false` | Resolves in command dir | **Rejected** — error shown to user |

The variable substitution itself still happens regardless — the check is on the **resolved path**, not the template. This means `${NIMBLE_COMMAND_DIR}/hello.sh` would still work even with `allow_external_paths: false` because it resolves to inside the command directory.

### Positives

1. **Removes the security concern.** The previous analysis flagged "security boundary erosion" as the main worry. This variant neutralizes it: users who care about containment flip one setting and get the same guarantees as today.

2. **Enabled-by-default is the right call.** Nimble is a personal productivity tool. The user writes every YAML and env file. Defaulting to "allow" follows the principle of least surprise — you set up `${SHARED_SCRIPTS}/tool.sh`, it works. No mysterious "permission denied" on first use.

3. **Consistent with `allow_duplicates` pattern.** The existing settings already follow this shape: a permissive default with an opt-in restriction. `allow_duplicates: true` by default, `allow_external_paths: true` by default — same pattern, easy to explain.

4. **Corporate/shared deployments.** An admin pre-seeding a config directory for a team can set `allow_external_paths: false` to enforce that all scripts stay co-located. The setting acts as a policy knob.

### Concerns

1. **The toggle is backwards from typical security defaults.** Security best practice says "deny by default, allow explicitly." But for a personal desktop tool this would create friction — every user who wants shared scripts must first discover and flip a setting. The target audience isn't running untrusted configs.

2. **Setting name matters a lot.** `allow_external_paths` is clear but verbose. Alternatives considered:
   - `allow_remote_scripts` — "remote" implies network, misleading
   - `restrict_scripts_to_command_dir` — double negative when false
   - `allow_external_paths` — wins: short, accurate, parallels `allow_duplicates`

3. **Enforcement boundary question.** Does the setting also restrict `env.yaml` loading paths? Or just `script:`/`list:` resolution? Recommendation: only `script:` and `list:` field resolution. The env system is already unconstrained (global env.yaml is at config root, not in a command dir).

4. **Mixed messaging in docs.** Documenting "scripts can be anywhere! (unless you disable it)" adds a conditional to every explanation. Mitigation: document co-located as the primary pattern, mention external paths as an advanced option, and cover the setting in a short "restricting script paths" section.

## Config impact

### settings.yaml

```yaml
# settings.yaml — new field
# When true (default), script: and list: fields can reference files anywhere
# via ${VAR} substitution. When false, they must resolve to the command
# directory (co-located only).
allow_external_paths: true
```

### Error when disabled

```
Error running "copy uuid": script path "/Users/me/scripts/uuid.sh"
resolves outside the command directory. Set allow_external_paths: true
in settings.yaml to allow external script paths.
```

The error message tells the user exactly what to do — no guessing.

### No config change needed for co-located

```yaml
# This works identically regardless of the setting
phrase: say hello
title: Say hello
action:
  type: dynamic_list
  config:
    script: hello.sh    # plain name → same directory → always allowed
    arg: optional
    item_action: paste_text
```

## Opinion

**Positive — this is the right design if we implement remote scripts at all.**

The toggle eliminates the strongest objection (security erosion) while keeping the default experience frictionless. The pattern mirrors `allow_duplicates` perfectly — permissive by default, restrictable by intent.

The only people who would set `false` are those who actively care about containment (corporate deployments, users managing shared machines). Forcing everyone else to opt-in would be hostile UX for a personal productivity tool.

**Compared to the base proposal (no toggle):** Strictly better. The toggle costs ~15 lines of code (one bool in AppSettings, one check in the path resolver) and completely addresses the security concern.

**Recommendation:** If you decide to ship remote scripts, ship it with this toggle. Don't ship remote scripts without it.

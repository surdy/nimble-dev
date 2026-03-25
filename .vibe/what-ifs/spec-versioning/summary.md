# What If: Independent Spec Versioning for Agents

## Question

Should the spec version (governing YAML schema, env vars, action types, script interface) be independent of the app version? How should agents track which spec version they target?

## Current State

| Thing | Version | Where |
|-------|---------|-------|
| App | `0.1.0` | `package.json`, `tauri.conf.json` |
| Spec | `1` (integer) | `.github/agents/nimble-spec.yaml` → `spec_version: 1` |
| Agents | No version field | `nimble-command.agent.md`, `nimble-script.agent.md` |

### What already exists

- `nimble-spec.yaml` has `spec_version: 1` and a `changelog:` section
- Rule 12a in `copilot-instructions.md` says: "Bump `spec_version` if a field is removed or its type/semantics change"
- Both agent files reference the spec via "Read `.github/agents/nimble-spec.yaml`"
- `nimble-conventions.md` also references the spec update requirement
- The changelog has two entries (initial spec, TSV format change) but both are `version: 1` — the TSV change probably should have bumped to `2`

## Recommendation: Independent, Integer-Based Spec Version

**Yes — keep spec versioning independent of app versioning.**

### Why

- They evolve at different rates: UI fixes, performance work, and bug fixes bump the app version without touching the spec
- Schema additions (new action types, new env vars) change the spec without necessarily warranting an app version bump
- Agents need a stable contract — "I was written for spec v3" is clearer than "I was written for app v0.3.2"

### Versioning scheme

| Versioning | Format | Tracks | Lives in |
|------------|--------|--------|----------|
| **App version** | Semver (`0.1.0`) | Builds, releases, bug fixes | `package.json`, `tauri.conf.json` |
| **Spec version** | Integer (`1`, `2`, `3`…) | Schema, action types, env vars, script interface | `nimble-spec.yaml` |
| **Agent version** | `spec_version` reference | Which spec the agent was written against | Each `.agent.md` file |

### When to bump spec version

| Change Type | Bump? | Example |
|-------------|-------|---------|
| New action type added | ✅ Yes | Adding `script_action` |
| New env var added | ✅ Yes | Adding `NIMBLE_VERSION` |
| Field removed or renamed | ✅ Yes | Renaming `list` → `list_file` |
| Field semantics changed | ✅ Yes | `static_list` file format YAML → TSV |
| New optional field added | ✅ Yes | Adding `env:` inline block |
| Settings schema changed | ✅ Yes | New `allow_external_paths` setting |
| Bug fix in how a field is parsed | ❌ No | Fixing edge case in path resolution |
| Documentation-only update | ❌ No | Clarifying a field description |

### Implementation plan

#### 1. Add `spec_version` field to agent files

In each `.agent.md` frontmatter (or as a comment at the top):
```markdown
---
spec_version: 2
---
```

This declares which spec version the agent was written for.

#### 2. Tighten bump rules

Current rule 12a only bumps on removals or type changes. Broaden to: **bump on any additive schema change** (new action types, new env vars, new config fields). This ensures agents are always re-evaluated when the spec grows.

#### 3. Keep changelog entries honest

Each changelog entry should have a **unique version number**:
```yaml
changelog:
  - version: 1
    date: "2025-07-14"
    changes:
      - "Initial spec — all six action types, settings, env vars, contexts."
  - version: 2
    date: "2026-03-21"
    changes:
      - "Static list file format changed from YAML to TSV (.tsv extension)."
```

#### 4. Sync check

When the spec version bumps, all `.agent.md` files must be reviewed and updated:
- Re-read the spec diff
- Update any instructions that reference changed fields
- Bump `spec_version` in agent frontmatter

This is already partially covered by rule 12a but should be made explicit for agents.

## What this does NOT cover

- Runtime compatibility checks (the app doesn't read agent files)
- Automated validation that agents match the spec
- Publishing agents to a registry (not applicable yet)

## Opinion

The current foundation is solid — `spec_version` and `changelog` already exist in the spec file. The missing pieces are:

1. **Agents don't declare which spec version they target** — add a `spec_version` field to each agent
2. **Bump threshold is too conservative** — additive changes should also bump, not just removals
3. **Changelog version numbers should be unique** — the TSV change should be `version: 2`

This is a small, low-risk change that makes the contract between spec and agents explicit.

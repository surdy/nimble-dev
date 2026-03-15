# What If: Per-Command Cache/Data Built-in Variables

_Date: 2026-03-14_

## Question

Should Nimble add two built-in script environment variables for per-command storage paths:

- `NIMBLE_COMMAND_CACHE_DIR` (volatile cache location)
- `NIMBLE_COMMAND_DATA_DIR` (persistent data location)

Given current and planned roadmap context:
- Stage 28: built-in script env variables
- Stage 29: user-defined script variables
- Stage 30: debug mode with `NIMBLE_DEBUG`

---

## 1) Recommendation

**Recommendation: Add, but in a phased rollout starting after Stage 28 baseline ships.**

- **Now (Stage 28):** Ship the minimal built-ins already planned (`NIMBLE_CONTEXT`, `NIMBLE_PHRASE`, `NIMBLE_CONFIG_DIR`, `NIMBLE_COMMAND_DIR`, `NIMBLE_OS`, `NIMBLE_VERSION`).
- **Next (new follow-up stage):** Add `NIMBLE_COMMAND_CACHE_DIR` and `NIMBLE_COMMAND_DATA_DIR` once the base env contract is stable.

### Why not in the first minimal set?

- Cache/data dirs are useful but introduce lifecycle and migration decisions that deserve dedicated design.
- Keeping Stage 28 small lowers release risk and documentation load.
- A follow-up stage can include cleanup policy and testing rigor without delaying baseline built-ins.

---

## 10) Final Go/No-Go + Trade-offs

### Decision: **Go (phased)**

- **Go for the concept:** It solves real script UX and reliability issues.
- **Go later, not immediately:** Add after Stage 28 baseline.

### Explicit trade-offs

**Pros**
- Scripts get stable, app-provided storage locations.
- Reduces ad hoc path handling inside scripts.
- Encourages correct split between ephemeral cache and persistent data.
- Better portability across macOS/Linux/Windows.

**Cons**
- Need deterministic naming and persistence semantics.
- Risk of orphaned storage directories when commands move/rename.
- Added docs complexity and user expectation management.

### Phased rollout plan

1. **Phase A (already planned):** Stage 28 minimal built-ins only.
2. **Phase B:** Add `NIMBLE_COMMAND_CACHE_DIR` and `NIMBLE_COMMAND_DATA_DIR` with deterministic path strategy and no auto-deletion.
3. **Phase C:** Optional tooling for manual cleanup and observability (size reports, stale-dir hints), but still avoid automatic destructive cleanup.

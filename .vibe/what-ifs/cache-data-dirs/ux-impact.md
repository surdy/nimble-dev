# UX, Risk, Testing, and Documentation Impact

## 4) Lifecycle and cleanup

### Cache invalidation policy

- Nimble should define cache as **best-effort disposable**.
- Scripts own cache correctness; Nimble provides location only.
- Recommend script pattern: check freshness timestamps and rebuild stale cache.

### Auto-deletion policy

**Recommendation: do not auto-delete by default.**

Rationale:
- Automatic cleanup can remove data scripts still expect.
- Nimble cannot reliably infer script-specific retention semantics.

Safer alternatives:
- Manual cleanup command in future (`ctx storage prune`) with preview/dry-run.
- Optional stale directory report in debug logs.

For persistent data:
- Treat as user-owned durable state.
- Never auto-delete unless user explicitly requests cleanup.

---

## 5) Security and privacy risks

### Risks

- Path traversal in directory creation logic
- Symlink abuse if canonicalization is inconsistent
- Sensitive data accidentally written to cache paths
- Verbose logs leaking storage paths or filenames that reveal user context

### Mitigations

- Never use user input directly for folder names; use deterministic hashed IDs.
- Canonicalize and enforce root containment before creating/writing directories.
- Keep `NIMBLE_COMMAND_*_DIR` values out of default non-debug logs where possible.
- In debug mode, avoid logging file contents or user-defined env values.

Residual risk:
- Scripts run with user privileges and may still misuse paths. Clear docs and examples are essential.

---

## 6) DX/UX implications

### What improves for users

- Easier script authoring: predictable storage paths across machines.
- Cleaner scripts: no hard-coded OS-specific path logic.
- Better reliability for large/slow external integrations.

### How scripts should use each variable

- `NIMBLE_COMMAND_CACHE_DIR`: temporary, recreatable artifacts (API cache, parsed indexes).
- `NIMBLE_COMMAND_DATA_DIR`: persistent state (cursor/checkpoint, user preferences for that command).

### Common misuse patterns and safeguards

Misuse:
- Writing durable state to cache directory.
- Writing large binary blobs to data directory without bounds.
- Assuming cache survives forever.

Safeguards:
- Provide docs checklist and examples.
- Add debug warnings for unusually large cache directories (future).
- Encourage scripts to version their own data files.

---

## 8) Testing strategy

### Unit tests

- Deterministic directory ID generation from command identity.
- Phrase/title edits do not change computed directories.
- File move/rename changes computed directories.
- Root containment and canonicalization enforcement.
- Variables injected in both `run_script()` and `run_script_values()`.

### Integration tests

- Cross-platform path roots resolve correctly (macOS/Linux/Windows).
- Directory creation on first script run works and is idempotent.
- Debug logging does not leak sensitive values.

### Regression tests

- Existing scripts still run with no dependency on new variables.
- Stage 28 built-ins unaffected.

---

## 9) Documentation changes required

Files to update if feature is implemented:

- `docs/using/advanced/writing-scripts.md`
  - Add a section: cache vs data, retention expectations, examples.
- `docs/using/config-directory.md`
  - Add storage roots and ownership/cleanup guidance.
- `docs/using/advanced/dynamic-list.md`
  - Mention availability of storage built-ins for heavy script workflows.
- `docs/using/advanced/script-action.md`
  - Same note with concrete script_action examples.

Docs should explicitly state:
- Built-in variables are app-provided and read-only from script perspective.
- Nimble does not auto-clean persistent data.
- Cache may be pruned by user tooling in future.

---

## Concise decision summary

- **Overall:** Positive feature with real workflow value.
- **Timing:** After Stage 28 minimal built-ins.
- **Critical design call:** deterministic per-command IDs based on command file identity.
- **Operational stance:** no automatic deletion initially; provide future manual cleanup tooling.

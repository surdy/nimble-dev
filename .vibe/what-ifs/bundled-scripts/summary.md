# What If: Bundled Scripts — Scripts Co-located with Command YAML

_Date: 2026-03-14_

## Question

What if the script referenced by a `dynamic_list` or `script_action` command lived alongside its command YAML in the same `commands/` subdirectory, instead of always living in the global `scripts/` directory?

## Key Findings

- **Self-contained "command packages"** — one subdirectory holds everything needed for a command
- **Eliminates the mental disconnect** between editing a command YAML in `commands/` and its script in `scripts/`
- **Breaks the current security boundary** — today, `script:` must be a plain filename (no `/`, `\`, `..`) resolved against `scripts/`. Co-located scripts would require path resolution relative to the command file, introducing more complex path-safety validation
- **Shared scripts become ambiguous** — when two commands in different subdirectories use the same script, where does it live? Duplication or a fallback to `scripts/` needed
- **File watcher complexity** — the watcher currently ignores non-YAML files in `commands/`; it would need to understand that executable files can live there too

## Recommendation

Support both: **relative `script:` paths within `commands/`** as an opt-in alongside the existing global `scripts/` lookup. This is additive — no existing configs break. The `scripts/` directory remains the default lookup location.

## Opinion

Mixed. The "command package" idea is appealing for distribution and self-containment. But the security trade-off is real — today's flat `scripts/` directory with plain-filename-only enforcement is simple and auditable. Co-locating scripts in `commands/` means relaxing path restrictions, which adds surface area. Worth doing only if there's strong user demand for portable/shareable command bundles.

## Files in This Analysis

- `summary.md` — this file
- `config-examples.md` — before/after YAML and directory layout examples
- `ux-impact.md` — detailed user impact analysis

# What If: Inline Static Lists in Command YAML

_Date: 2026-03-14_

## Question

What if static list items could be embedded directly in the command YAML file, instead of always referencing a separate file in `lists/`?

## Key Findings

- **Reduces friction for small lists** — no need to create a second file for 3–5 items
- **Self-contained commands** — one file is the single source of truth for both config and data
- **Should be additive, not a replacement** — external `list:` references must remain for shared/large lists
- **Validation rule**: `items` and `list` are mutually exclusive — providing both is an error

## Recommendation

**Support both `items` (inline) and `list` (external file).** This is a pure additive change — no existing configs break. New users get a simpler path; power users keep the external file pattern.

## Opinion

Clearly positive if done as "both, not either/or." The current two-file requirement is unnecessary friction for trivial lists. The `lists/` directory becomes optional rather than mandatory, which aligns with Nimble's principle of simplicity.

## Files in This Analysis

- `summary.md` — this file
- `config-examples.md` — before/after YAML config examples
- `ux-impact.md` — detailed user impact analysis
- `static-list-docs.md` — how the static list documentation would read with inline support

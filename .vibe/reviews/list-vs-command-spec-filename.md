# List vs Command Spec — Filename Convention
_Date: 2026-03-13_

Under Option B (co-locate list files next to command specs), both types of file end in `.yaml`. A directory like:

```
shortcuts/
  team-email.yaml
  team-emails.yaml
```

…gives no visual signal about which file is a command spec and which is list data. Users copying, deleting, or renaming have to open files to know what they are.

---

## Options

### Option 1 — Double extension: `.list.yaml` *(recommended)*

List files use a compound extension; command specs use plain `.yaml`:

```
shortcuts/
  team-email.yaml          ← command spec
  team-emails.list.yaml    ← list data
```

The loader globs `*.yaml` (excluding `*.list.yaml`) for command specs, and `*.list.yaml` for list data.

| Pro/Con | Note |
|---------|------|
| Pro | Scannable at a glance; distinction is enforced by the loader, not just convention |
| Pro | YAML syntax highlighting works without any editor config (still ends in `.yaml`) |
| Pro | Familiar pattern — developers know `.test.ts`, `.d.ts`, `.config.js` |
| Pro | Extensible to future types: `.data.yaml`, `.template.yaml` |
| Con | Slightly verbose — e.g. `team-emails.list.yaml` — dissolved by choosing short names |

### Option 2 — Separate extension: `.list`

List files drop `.yaml` entirely:

```
shortcuts/
  team-email.yaml
  team-emails.list
```

| Pro/Con | Note |
|---------|------|
| Pro | Clean visually |
| Con | No YAML syntax highlighting by default — users must configure their editor |

### Option 3 — Filename prefix sigil (`_` or `@`)

```
shortcuts/
  team-email.yaml
  _team-emails.yaml
```

| Pro/Con | Note |
|---------|------|
| Pro | Sorts data files together alphabetically |
| Con | Convention only — no programmatic enforcement unless the loader explicitly checks; fragile if users forget the prefix |

### Option 4 — `_data/` local subdirectory

```
shortcuts/
  work/
    jira.yaml
    _data/
      jira-issues.yaml
```

| Pro/Con | Note |
|---------|------|
| Pro | Preserves co-location with clear separation; `_data/` is a recognisable pattern (Jekyll, Hugo) |
| Con | Adds a directory level even for the simplest case |

---

## Recommendation

**Option 1 (`.list.yaml`)** — enforced by the loader, YAML highlighting works out of the box, familiar double-extension pattern, extensible to future data types.

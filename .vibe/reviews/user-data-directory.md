# User Data Directory
_Date: 2026-03-13_

Scripts often need to read dynamic, reusable data (contacts, project lists, templates) that is user-specific, updated externally, and shared across multiple scripts/commands. This is distinct from bundle data (static, ships with the command) and from `lists/` (static_list action only). The question is where this data lives and how scripts find it.

---

## Options

### Option 1 — `data/` subdirectory inside the app config dir

```
ContextActions/
  settings.yaml
  shortcuts/
  scripts/
  data/
    contacts.csv
    projects.json
```

| Pro/Con | Note |
|---------|------|
| Pro | One directory to know about; no new path concept |
| Pro | Straightforward default |
| Con | Mixes dynamic user-generated data with app config — semantically messy |
| Con | Path is buried (`~/Library/Application Support/ContextActions/data/`) — awkward for external tools to write to |

### Option 2 — Separate configurable directory with sensible per-platform default

| Platform | Default |
|----------|---------|
| macOS | `~/ContextActions/` or `~/Documents/ContextActions/` |
| Linux | `$XDG_DATA_HOME/ContextActions/` (~`~/.local/share/ContextActions/`) |
| Windows | User's Documents folder |

Configurable in `settings.yaml` as `data_dir: ~/my-data`.

| Pro/Con | Note |
|---------|------|
| Pro | User can point it at wherever external tools already write data |
| Pro | Follows XDG distinction between config and data on Linux |
| Pro | More accessible location than buried app support dirs |
| Con | Two directories to know about and explain |
| Con | Adds a settings field most users will never touch |

### Option 3 — Injected environment variable (`CONTEXTS_DATA_DIR`)

The launcher injects `CONTEXTS_DATA_DIR` into every script invocation, pointing at the data directory. Scripts reference data portably:

```sh
contacts="$CONTEXTS_DATA_DIR/contacts.csv"
```

| Pro/Con | Note |
|---------|------|
| Pro | Scripts are portable across machines — never hardcode a platform path |
| Pro | User can relocate the data dir (Dropbox, NAS, etc.) with one settings change; all scripts adapt automatically |
| Pro | Combines with Option 1 or 2 for the actual location decision |
| Con | Scripts must use the env var idiom — slightly more to document |
| Con | Launcher must inject the var at script spawn time (minor implementation concern) |

---

## Recommendation

All three together, layered:

1. **Default location:** `data/` inside the app config dir (Option 1) — simple, zero config for the common case
2. **`data_dir` in `settings.yaml`** (Option 2) — power users can point at Dropbox, NAS, or wherever external tools already write
3. **`CONTEXTS_DATA_DIR` injected into every script spawn** (Option 3) — scripts never hardcode paths; portability is automatic

### Distinction from bundle data

| | Bundle data | User data |
|---|---|---|
| Location | Inside the bundle dir | `data/` (or configured path) |
| Written by | Author, committed with bundle | User, external tools, cron jobs |
| Scope | One bundle | Any script across any command |
| Dynamic | No (static) | Yes |

These are complementary, not competing. A script can read from both.

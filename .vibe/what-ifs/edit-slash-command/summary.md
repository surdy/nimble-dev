# What If: `/edit` Slash Command for Editing Command Files

## Question

Should we add a built-in `/edit` slash command that lets users quickly find and open the files behind a Nimble command (YAML, scripts, env, lists)?

## Context

Users currently have no in-app way to jump from a command they use to the files that define it. They must manually navigate `~/Library/Application Support/Nimble/commands/` to find and edit command files. A `/edit` slash command would close this gap.

## Architecture Analysis

### Current slash command system

- Built-in commands are **hardcoded** in `+page.svelte` as `Command[]` with `type: "builtin"`
- `BuiltinConfig` in `types.ts` defines the action union (`"ctx_set" | "ctx_reset"`)
- When input starts with `/`, only built-in commands are shown
- `/ctx` is **entirely frontend** — no Tauri backend calls needed

### Why `/edit` is different from `/ctx`

Unlike `/ctx`, the `/edit` command requires **backend support** because it needs to:

1. Resolve a command phrase → its directory path on disk
2. Discover related files (YAML, scripts, `env.yaml`, list `.tsv`/`.txt`)
3. Filter out binary files
4. Open files in the user's editor or reveal the folder in Finder

### Implementation layers

| Layer | Work |
|-------|------|
| **Rust backend** | New Tauri command `get_command_files(phrase)` → returns list of file paths for that command. New command to `open_in_editor(paths)` or `reveal_in_finder(path)` |
| **Types** | Add `"edit"` (or similar) to `BuiltinConfig.action` union |
| **Frontend** | Add `/edit` built-in command. When selected, show matched commands → on select, open their files |

### Related files a command can have

| File Type | Location | Purpose |
|-----------|----------|---------|
| Command YAML | `commands/<name>/<name>.yaml` | Main command definition |
| Script | `commands/<name>/<script>.sh` (or .py, .js, etc.) | Referenced by `script:` field |
| Sidecar env.yaml | `commands/<name>/env.yaml` | Scoped environment variables |
| Static list data | `commands/<name>/<list>.tsv` or `.txt` | Data for `static_list` actions |

### Existing infrastructure

- **`tauri-plugin-opener`** is already in the project — provides `open_url()` and `open_path()`
- The backend already knows every command's directory via `NIMBLE_COMMAND_DIR`
- The `load_from_dir` function already walks command directories and knows file associations

## UX Options

### Option A: `/edit <phrase>` → open all files in default editor

User types `/edit say hello`, Nimble opens all related text files (YAML, script, env.yaml) in the system default editor. Simple, direct.

**Pro:** Fastest path from intent to editing.
**Con:** May open too many files; relies on OS file associations being set to a code editor.

### Option B: `/edit` → filterable command list → reveal folder

User types `/edit`, sees a list of all commands. Selecting one reveals the command folder in Finder/Explorer. User opens what they need.

**Pro:** Familiar file management; user has full control.
**Con:** Two extra steps (select command, then open file from Finder).

### Option C: `/edit <phrase>` → show related files as a list → user picks

User types `/edit say hello`, sees a list like:
- `say-hello.yaml`
- `hello.sh`
- `env.yaml`

Selecting a file opens it in the default editor.

**Pro:** Granular control; stays in the Nimble UX.
**Con:** More implementation work (dynamic list from backend).

## Recommendation

**Option C** gives the best balance of power and UX consistency — it reuses the familiar Nimble list pattern, keeps the user in the launcher, and avoids opening unwanted files. Option B is a simpler fallback if scope needs trimming.

## Open Questions

1. Should `/edit` also offer a "Reveal in Finder" option alongside individual files?
2. Should it support editing commands from `example-config/` (repo) or only the live config dir?
3. What should happen for commands with no editable files (e.g., single-file YAML with no scripts)?
4. Should the command also allow creating new files (e.g., adding an `env.yaml` sidecar)?

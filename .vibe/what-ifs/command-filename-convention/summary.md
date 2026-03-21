# What If: Different filename patterns for command YAML vs list YAML

## The Problem

Today, `collect_yaml_files()` in `commands.rs` walks the entire `commands/` directory tree and treats **every** `.yaml`/`.yml` file as a command. When co-located files exist — static list data files (`team-emails.yaml`), sidecar env files (`env.yaml`) — Nimble tries to deserialise them as `Command` structs and prints parsing errors to the log:

```
[nimble] could not parse commands/examples/show-team-emails/team-emails.yaml: ...
[nimble] could not parse commands/examples/show-user-env/env.yaml: ...
```

These are harmless (the app still works), but they're noisy, confusing, and make it harder to spot real errors.

## Options Explored

### Option A — Reserved filename: `env.yaml`; distinguish commands by naming convention

Introduce a naming convention where **command files** use a specific suffix or pattern (e.g. `*.cmd.yaml`) and everything else is ignored.

| Variant | Command file | List file | env file |
|---------|-------------|-----------|----------|
| A1: `.cmd.yaml` suffix | `open-google.cmd.yaml` | `team-emails.yaml` | `env.yaml` |
| A2: `.command.yaml` suffix | `open-google.command.yaml` | `team-emails.yaml` | `env.yaml` |

**Pros:**
- Unambiguous — only files matching the suffix are parsed as commands
- List files, env files, READMEs, and any other YAML can coexist freely
- No log noise at all

**Cons:**
- **Breaking change** — every existing user must rename all their command files
- Longer filenames; more friction when creating commands
- Feels heavy-handed — the convention exists only to solve a log noise problem
- Less intuitive to newcomers ("why do I need `.cmd.yaml`?")

### Option B — Skip known non-command filenames

Keep `*.yaml` as the command file format. Skip files named `env.yaml` (already a known sidecar type) and skip files whose content starts with `- ` or `[` (YAML arrays, i.e., list files).

**Pros:**
- No breaking change — existing configs work as-is
- Eliminates the most common log noise sources

**Cons:**
- Fragile heuristic — content sniffing is error-prone
- Doesn't scale if new non-command YAML types are added
- `env.yaml` skip is clean, but array-sniffing feels hacky

### Option C — Skip `env.yaml` by name; silently ignore parse failures for non-command YAML

Already skip `env.yaml` (it's a known type). For all other YAML files, attempt to parse as `Command`; if parsing fails, **don't log an error** (or log at debug/trace level only).

**Pros:**
- No breaking change
- Simplest code change
- Works immediately

**Cons:**
- Hides *real* parse errors in broken command files — users won't know why their command isn't loading
- Debugging becomes harder

### Option D — Filename convention for non-command files (inverted approach)

Keep `*.yaml` as command files. Introduce a convention for **non-command** files: list files use a distinct extension or prefix that tells the loader to skip them.

| Variant | Command file | List file | env file |
|---------|-------------|-----------|----------|
| D1: `.list.yaml` for lists | `open-google.yaml` | `team-emails.list.yaml` | `env.yaml` |
| D2: `_` prefix for data | `open-google.yaml` | `_team-emails.yaml` | `env.yaml` |

**Pros:**
- No breaking change to command files (the common case)
- Only list files need renaming (rare, advanced feature)
- Clear intent — "this is data, not a command"

**Cons:**
- Still a breaking change for existing `static_list` users (list file naming changes)
- `list:` field resolution would need to know about the new extension
- Adds cognitive load ("which extension do I use?")

### Option E — Skip `env.yaml` by name; log non-command parse failures as warnings with a hint

Skip `env.yaml` explicitly. For all other files that fail to parse as a `Command`, log a **warning** (not an error) with a helpful hint suggesting the file might be a list or data file:

```
[nimble] skipping commands/examples/show-team-emails/team-emails.yaml — not a valid command file (is this a list file?)
```

**Pros:**
- No breaking change
- Acknowledges the issue without hiding real errors
- Users can still see which files were skipped and why
- Simple implementation

**Cons:**
- Still produces log lines (just better ones)
- Doesn't eliminate the noise entirely

### Option F — Skip `env.yaml` by name; attempt parse, downgrade to `debug!()` on failure

Skip `env.yaml` explicitly. For all other files, attempt to parse as `Command`; if parsing fails, log at `debug` level (invisible by default, visible with `RUST_LOG=debug` or a future `NIMBLE_DEBUG` mode).

**Pros:**
- No breaking change
- Zero noise in normal operation
- Errors still accessible for debugging
- Clean separation: valid commands load, invalid files silently ignored, debug mode reveals everything

**Cons:**
- Users with genuinely broken command files won't see errors unless they enable debug mode
- Could be paired with a startup summary ("Loaded 12 commands from 15 files, 3 skipped") to surface awareness without noise

## Opinion

**Option F (skip `env.yaml` + debug-level logging) is the best fit**, ideally paired with a startup summary line like:

```
[nimble] loaded 12 commands (3 files skipped, run with NIMBLE_DEBUG=1 for details)
```

This gives you:
1. **Zero noise** in normal operation — the most common complaint resolved
2. **No breaking change** — existing configs work as-is
3. **Debuggability** — `NIMBLE_DEBUG=1` reveals exactly which files were skipped and why
4. **Awareness** — the summary line tells users that files were skipped, prompting them to investigate if their command count looks wrong

The `.cmd.yaml` convention (Option A) is the "purest" solution but imposes too much friction for what is fundamentally a log noise problem. It would make sense if Nimble were starting fresh, but with existing users and documentation, the migration cost isn't justified.

I'd **avoid** content-sniffing (Option B) entirely — it's fragile and surprising. And I'd avoid silencing errors completely (Option C) because it makes debugging genuinely broken commands much harder.

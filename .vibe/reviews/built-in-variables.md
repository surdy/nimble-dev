# Review: Built-in Environment Variables for Scripts
_Date: 2026-03-14_

## Overview

Built-in variables are environment variables that Nimble injects into every script subprocess it spawns. They give scripts access to app-level context (active context, config directory, command phrase, OS platform, etc.) without requiring the user to pass them as positional arguments. This is a low-friction way to make scripts more powerful while preserving the existing security model — scripts remain read-only consumers of information.

---

## Proposed variables

| Variable | Value | Rationale |
|----------|-------|-----------|
| `NIMBLE_CONTEXT` | Active context string, or empty | Scripts can vary behaviour by context without the user passing it as an arg |
| `NIMBLE_PHRASE` | The command phrase that triggered this script | Useful when one script is shared by multiple commands |
| `NIMBLE_CONFIG_DIR` | Absolute path to the Nimble config root | Scripts can read sibling data files, settings, etc. |
| `NIMBLE_COMMAND_DIR` | Absolute path to the co-located command directory | The most useful one — scripts can find data files next to themselves |
| `NIMBLE_OS` | `macos`, `linux`, or `windows` | Scripts can branch on platform without calling `uname` |
| `NIMBLE_VERSION` | App version string (e.g. `1.2.0`) | Lets scripts check compatibility |

---

## Implementation strategy

**Where to inject:** In `run_script()` and `run_script_values()` in `src-tauri/src/commands.rs`, right after building the `std::process::Command` and before calling `.spawn()`. The `Command::env()` or `Command::envs()` method injects variables into the child process without affecting the launcher's own environment.

```rust
// Pseudocode — not a real change
cmd.env("NIMBLE_CONTEXT", context)
   .env("NIMBLE_PHRASE", phrase)
   .env("NIMBLE_COMMAND_DIR", command_dir.to_string_lossy().as_ref())
   .env("NIMBLE_CONFIG_DIR", config_dir.to_string_lossy().as_ref())
   .env("NIMBLE_OS", std::env::consts::OS)
   .env("NIMBLE_VERSION", env!("CARGO_PKG_VERSION"));
```

**What needs to flow down:** Currently `run_script` and `run_script_values` only receive `command_dir`, `script_name`, and `arg`. To inject `NIMBLE_CONTEXT` and `NIMBLE_PHRASE`, these values need to be passed from the frontend → Tauri command → `run_script*`. Two options:

1. **Add parameters to the Tauri commands** — `context: Option<String>` and `phrase: String` added to `run_dynamic_list` and `run_script_action`. The frontend already has both values at call time.
2. **Bundle into a struct** — Create a `ScriptEnv` struct carried through the call chain. Cleaner if the variable count grows.

Option 1 is simpler and sufficient for the initial set.

---

## Usage from scripts

**Shell (bash/zsh):**
```bash
#!/bin/bash
echo "Running in context: $NIMBLE_CONTEXT"
echo "Config at: $NIMBLE_CONFIG_DIR"

if [ "$NIMBLE_OS" = "macos" ]; then
    open "https://example.com"
fi
```

**PowerShell:**
```powershell
$ctx = $env:NIMBLE_CONTEXT
Write-Output "Context is: $ctx"
```

**Python:**
```python
import os
context = os.environ.get("NIMBLE_CONTEXT", "")
command_dir = os.environ["NIMBLE_COMMAND_DIR"]
```

---

## Naming convention: UPPER_CASE ✅

**Strongly recommend `UPPER_CASE` with a `NIMBLE_` prefix.** Reasons:

- **Convention:** Environment variables are uppercase on every major OS and in every major tool (POSIX, Windows, Docker, CI systems, Kubernetes). `PATH`, `HOME`, `CI`, `GITHUB_TOKEN` — all caps. Lowercase env vars exist but are unconventional and look like typos.
- **Visibility:** Uppercase makes it immediately obvious you're referencing an environment variable, not a local shell variable (bash in particular treats `lower_case` as local-by-convention).
- **Namespacing:** The `NIMBLE_` prefix prevents collisions with system variables (`CONTEXT` alone is too generic). This follows the pattern of `GITHUB_*`, `DOCKER_*`, `AWS_*`.
- **Cross-platform:** PowerShell on Windows is case-insensitive for env vars, but shell on Unix is case-sensitive. Starting with uppercase avoids surprises when users port scripts between platforms.

---

## YAML schema impact

None. Variables are injected automatically by the app into every script. No configuration required from the user's side — every script automatically gets the variables. Users who don't need them simply ignore them.

---

## Security considerations

- `NIMBLE_CONFIG_DIR` and `NIMBLE_COMMAND_DIR` expose local paths. This is acceptable since scripts already run with the user's privileges and already know their own location via `$0` / `__file__`.
- No secrets are exposed — these are all non-sensitive operational values.
- The existing sandbox model is preserved: variables are read-only information; scripts still cannot trigger actions.

---

## Open questions

- Should `NIMBLE_INPUT` (the full raw typed text, before suffix extraction) be exposed? It would let scripts access everything the user typed, not just the positional arg.

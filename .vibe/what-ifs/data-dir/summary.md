# What If: Data Directory Variable for script_action

_Date: 2026-03-14_

## Question

What if `script_action` (and `dynamic_list`) supported a configurable data directory — a path within the user's home directory where external applications drop data, and the script can read from it? The directory path would be specified in the command YAML and passed to the script as an environment variable.

## The Problem Being Solved

Today, if a script needs data from another application (e.g., an export file, a JSON dump, a CSV), the script author must hard-code the path or rely on convention. There's no standard way for the command YAML to tell the script "look here for your data." This makes commands less portable — the same script works differently on different machines unless paths are manually aligned.

## Key Findings

- **Solves a real problem** — bridging data between external apps and Nimble scripts is a legitimate use case
- **Minimal config change** — one new optional field (`data_dir`) in the script config
- **Clean contract** — Nimble validates the directory exists and passes it as an env var; the script reads from it
- **Security consideration** — must restrict to paths within `$HOME` to prevent scripts from being pointed at system directories
- **Alternative considered** — a generic `env` map is more flexible but also more complex and harder to reason about

## Recommendation

Add an optional `data_dir` field to `script_action` and `dynamic_list` configs. Nimble resolves `~` to `$HOME`, validates the path is within `$HOME`, checks the directory exists, and passes the absolute path as the `NIMBLE_DATA_DIR` environment variable when spawning the script.

## Opinion

**Positive, with caveats.** This solves a concrete integration problem cleanly — the alternative (hard-coding paths in scripts) is brittle and non-portable. However, the `$HOME`-only restriction is important: without it, a command YAML could point a script at `/etc/` or `/var/` which feels wrong for a user-space tool. The feature is narrow enough to be easy to understand and hard to misuse.

A more generic `env` map would be more powerful but also introduces "configure anything" complexity that doesn't match Nimble's philosophy of simplicity.

## Files in This Analysis

- `summary.md` — this file
- `config-examples.md` — before/after YAML examples and the proposed spec
- `ux-impact.md` — detailed user perspective analysis

# Nimble — GitHub Copilot Instructions

## Project Overview
Contexts is a cross-platform desktop launcher (similar to Alfred, Spotlight, Raycast) that is primarily command-driven. Users type multi-word phrase commands and the launcher shows partial matches with suggested completions in real time.

## Architecture & Design Principles

### Command System
- Commands are **multi-word phrases** (e.g., "open google", "paste invoice template")
- As the user types, the launcher shows **partial matches** for known commands
- Each result displays:
  - **Title** (main text): a human-readable description provided by the user/plugin author
  - **Subtext**: possible completions or hints for the matched command
- The launcher does **not** launch desktop applications by default; app launching may be added later under a dedicated `launch` command

### Built-in Actions
All system actions the launcher can perform:
1. **Open URL** — navigate to a webpage in the default browser; optionally accepts a query parameter passed as a `{param}` placeholder in the URL
2. **Paste Text** — paste a predefined text string into the application that had focus before the launcher was invoked
3. **Copy Text** — copy a predefined text string to the clipboard without simulating a keystroke
4. **Static List** — display a named YAML list of items inline; each item can trigger `paste_text`, `copy_text`, or `open_url`
5. **Dynamic List** — run an external script and display its stdout as a list of items; supports `none`, `optional`, and `required` argument modes

### Script Extension System
- Users associate commands with executables in the `scripts/` config subdirectory (shell scripts, Python, Node.js, binaries, etc.)
- Scripts return plain text or a JSON array of `{ title, subtext }` objects
- The **action** tied to each result must be one of the launcher's built-in functions — scripts cannot directly perform system actions themselves
- Scripts are sandboxed: they do **not** have permission to modify the system or trigger actions; that responsibility belongs solely to the launcher's built-in layer

## Development Workflow Rules
1. **Commit and push after every meaningful step** with a clear, descriptive commit message. After every commit, run `cargo test --manifest-path src-tauri/Cargo.toml` and confirm all tests pass, then run `git push` to publish the commit to the remote.
2. **Add backend tests for every new Rust function**: whenever a new pure helper function or a new action type is added to the Rust backend, add corresponding `#[cfg(test)]` unit tests covering the happy path, edge cases, and expected error conditions. Tests for pure helpers go in `src-tauri/src/lib.rs`; tests for command loading / YAML parsing go in `src-tauri/src/commands.rs`.
3. **Ask questions instead of making assumptions** whenever requirements are ambiguous
4. **Keep README.md and user-facing documentation up to date** as features are built
5. **Do not write code** until requirements for that phase/feature are clearly understood
6. **Add a docs page for every new action type.** Create a dedicated page in `docs/actions/` covering the YAML schema, a minimal example, any parameter behaviour, and platform-specific requirements. Update `docs/actions/README.md` to link to the new page.
7. **Keep `docs/reference/config-directory.md` up to date** whenever a new subdirectory is introduced in the config directory (e.g. for scripts, settings, or any future data type). Add a dedicated section describing the subdirectory's purpose and any relevant file format notes.
8. **Keep `docs/guides/writing-scripts.md` up to date** whenever the `dynamic_list` script interface changes (output format, arg modes, security boundaries, or timeout behaviour). Also keep `docs/guides/configuring-commands.md` current with the full YAML schema for all action types.
9. **Update `example-config/` for every new action type or config feature.** Add at least one representative command YAML (and any required list/script files) to `example-config/` so the repository always contains a copy-pasteable reference that exercises every supported capability. Update `example-config/README.md` to document the new entry.
10. **Install live examples in the local app config directory after every new feature.** After implementing a new action type or config feature, add a working example directly to `~/Library/Application Support/Nimble/` (macOS) so it is immediately testable in the running app. Mirror the same files that were added to `example-config/`.
11. **Update `docs/roadmap.md` when a planned item is implemented.** Mark the corresponding roadmap item with a `[x]` checkbox and update its description to reflect the final implementation if it differs from the original wording.

## Tech Stack
- **Runtime / native shell**: Tauri 2 (Rust)
- **Frontend UI**: SvelteKit + TypeScript (rendered in the system webview via Tauri)
- **Build tooling**: Vite
- **Package manager**: npm
- **Rust toolchain**: stable via rustup

## Tech & Platform Notes
- Target: **cross-platform desktop** (macOS primary, Windows and Linux later)
- Prefer native performance for the UI layer (launcher must feel instant)
- Keep the core launcher (Rust/Tauri) and the plugin/script layer cleanly separated
- Frontend lives in `src/`, Rust/Tauri backend in `src-tauri/`

## Security Constraints
- Scripts/executables invoked by the launcher must be treated as **untrusted input processors only**
- No script should be able to trigger file system writes, network calls, or OS-level actions directly — all such actions flow through the launcher's verified built-in functions
- Validate and sanitise all inputs from scripts before acting on them
- Follow OWASP secure coding practices throughout

## File & Folder Conventions
- `bootstrap/` — project bootstrap documents and initial planning notes
- `docs/` — user-facing and developer documentation
- `src/` — SvelteKit frontend source
- `src-tauri/` — Rust/Tauri backend source
- `.vibe/` — decision logs (see rule 12 below)
- `.github/skills/nimble-authoring/` — Copilot skill for command and script authoring (`SKILL.md` + `nimble-spec.yaml`)

## Spec Rules
12a. **Keep `.github/skills/nimble-authoring/nimble-spec.yaml` in sync with the codebase.** Whenever a change is made to the command YAML schema, action types, settings schema, script interface, environment variable API, or context system, update the spec file to reflect the change. Add a changelog entry at the bottom with the date and a one-line description. Bump `spec_version` when: a field is removed/renamed, a type or semantics change, a new action type is added, a new env var is added, a new config field is added, or the script interface changes.

## Decision Logging Rules
12. **Log every meaningful trade-off or technology decision to `.vibe/decisions-details.md` and `.vibe/decisions.md`.** This applies to any choice where two or more reasonable approaches were considered — library selection, architectural patterns, API design, UI/UX trade-offs, security model choices, performance strategies, etc. **Also log any reversal**: when the user questions a previous decision and we switch to a different approach, record it as a dedicated entry (or amend the original entry) explaining what triggered the reversal and why the new route was chosen.

    **`.vibe/decisions-details.md`** — append one entry per decision using this exact structure:
    ```
    ## <Short title summarising the decision>
    _Date: YYYY-MM-DD_

    ### Options evaluated
    **Option A — <name>**
    - Pros: …
    - Cons: …

    **Option B — <name>**
    - Pros: …
    - Cons: …

    ### Reversal (include this section only when overturning a previous decision)
    _Original decision:_ "<title of previous entry>" chose <option> _(YYYY-MM-DD)_
    _Trigger:_ <what prompted the question or rethink>
    _Why we changed route:_ <concise rationale — what was wrong with the original choice>

    ### Decision
    <Which option was chosen and the core rationale in 1–3 sentences.>

    ### Risks & pitfalls
    - …
    ```

    **`.vibe/decisions.md`** — append one bullet per decision (title + one-line outcome only, no rationale or risks). Treat this file as an **append-only log** — never modify or remove an existing bullet. For reversals, add a new bullet at the end referencing the original decision; do not alter the original bullet:
    ```
    - **<Short title>** — <chosen option in a few words> _(YYYY-MM-DD)_
    - **<Short title> (reversal of: "<original title>" YYYY-MM-DD)** — switched from <old> to <new>; <one-line reason> _(YYYY-MM-DD)_
    ```

    Both files live in `.vibe/` at the repo root and are committed alongside the code change they relate to.

## Git & Commit Hygiene Rules
13. **Never create an "Initial plan" or placeholder commit.** Do not commit a plan,
    outline, or todo list as a standalone commit before writing any code. Planning is
    done in the conversation; only substantive code or documentation changes are committed.

14. **Never create merge commits on `main`.** All pull requests opened by Copilot must
    be merged using **squash merge** or **rebase merge** only. Do not use the default
    merge strategy that produces a `Merge pull request #N …` commit. When merging via
    the GitHub UI, always select "Squash and merge" or "Rebase and merge".

15. **Every commit must contain at least one meaningful file change.** A commit that
    only modifies metadata, adds an empty file, or produces no diff to any tracked file
    is forbidden. If there is nothing substantive to commit yet, do not commit.

16. **Commit messages must follow the Conventional Commits format.** Use one of:
    `feat`, `fix`, `docs`, `chore`, `refactor`, `test`, `ci`, `style`, `perf`.
    The subject line must be ≤ 72 characters and written in the imperative mood.
    A body is optional but recommended for non-trivial changes.

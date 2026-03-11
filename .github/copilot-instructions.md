# Context Actions — GitHub Copilot Instructions

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
6. **Classify new functionality as basic or advanced before writing docs.** Basic actions are self-contained commands a non-technical user can set up in under a minute (`open_url`, `paste_text`, `copy_text`). Advanced features require additional files, scripts, or configuration knowledge (`static_list`, `dynamic_list`). If it is unclear which category a new feature belongs to, ask the user before placing it. Once classified, add a dedicated page in the appropriate folder (`docs/using/basic/` or `docs/using/advanced/`) covering the YAML schema, a minimal example, any parameter behaviour, and platform-specific requirements. Update the corresponding landing page (`docs/using/basic/README.md` or `docs/using/advanced/README.md`) to link to the new page.
7. **Keep `docs/using/config-directory.md` up to date** whenever a new subdirectory is introduced in the config directory (e.g. for scripts, settings, or any future data type). Add a dedicated section describing the subdirectory's purpose and any relevant file format notes.
8. **Keep `docs/using/advanced/writing-scripts.md` up to date** whenever the `dynamic_list` script interface changes (output format, arg modes, security boundaries, or timeout behaviour). Also keep `docs/using/configuring-commands.md` current with the full YAML schema for all action types.
9. **Update `example-config/` for every new action type or config feature.** Add at least one representative command YAML (and any required list/script files) to `example-config/` so the repository always contains a copy-pasteable reference that exercises every supported capability. Update `example-config/README.md` to document the new entry.
10. **Install live examples in the local app config directory after every new feature.** After implementing a new action type or config feature, add a working example directly to `~/Library/Application Support/ContextActions/` (macOS) so it is immediately testable in the running app. Mirror the same files that were added to `example-config/`.
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

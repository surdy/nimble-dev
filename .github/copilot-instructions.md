# Contexts Launcher — GitHub Copilot Instructions

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

### Core Built-in Functions (Phase 1)
These are the only system actions the launcher itself can perform:
1. **Open URL** — navigate to a webpage in the default browser; optionally accepts a query parameter passed as a `param` variable in the URL
2. **Paste Text** — paste a predefined text string into the application that had focus before the launcher was invoked

### Plugin / Script Extension System (Phase 2+)
- Users can associate custom commands with external scripts or executables
- Scripts may accept **input parameters** and must return either:
  - Plain text
  - Structured JSON representing a list of results (each result has a title, subtext, and an associated action)
- The **action** tied to each result must be one of the launcher's built-in functions — scripts cannot directly perform system actions themselves
- Scripts are sandboxed: they do **not** have permission to modify the system or trigger actions; that responsibility belongs solely to the launcher's built-in layer

## Development Workflow Rules
1. **Commit after every meaningful step** with a clear, descriptive commit message. After every commit, run `cargo test --manifest-path src-tauri/Cargo.toml` and confirm all tests pass before proceeding.
2. **Add backend tests for every new Rust function**: whenever a new pure helper function or a new action type is added to the Rust backend, add corresponding `#[cfg(test)]` unit tests covering the happy path, edge cases, and expected error conditions. Tests for pure helpers go in `src-tauri/src/lib.rs`; tests for command loading / YAML parsing go in `src-tauri/src/commands.rs`.
3. **Ask questions instead of making assumptions** whenever requirements are ambiguous
3. **Keep README.md and user-facing documentation up to date** as features are built
4. **Do not write code** until requirements for that phase/feature are clearly understood
5. **Keep `docs/using/basic-functionality.md` up to date** whenever a new built-in action type is added to the launcher (currently: `open_url`, `paste_text`, `copy_text`, `static_list`; Stage 15 adds `dynamic_list`). Add a dedicated section covering the YAML schema, a minimal example, any parameter behaviour, and platform-specific requirements.
6. **Keep `docs/using/config-directory.md` up to date** whenever a new subdirectory is introduced in the config directory (e.g. for scripts, settings, or any future data type). Add a dedicated section describing the subdirectory's purpose and any relevant file format notes.
6. **Update `docs/using/` after Dynamic List (Stage 15) ships:** add a new `docs/using/script-extensions.md` covering how to write and register a `dynamic_list` command, the expected stdout format (plain text and JSON), the three `arg` modes (`none` / `optional` / `required`), security boundaries, and debugging tips. Also update `docs/using/basic-functionality.md` with a Dynamic List section, and update `docs/using/configuring-commands.md` with the `dynamic_list` action type and its YAML schema.

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
- `src/` — application source code (to be created when coding begins)
- `plugins/` — example and built-in plugin/script definitions (Phase 2+)

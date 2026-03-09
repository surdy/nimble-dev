# Ctx Launcher

A fast, cross-platform desktop launcher driven entirely by commands — think Alfred or Raycast, built around multi-word phrase commands with real-time partial matching and extensibility through sandboxed scripts.

---

## Tech Stack

- **[Tauri 2](https://tauri.app/)** — Rust-powered native shell (tiny binary, fast startup, strong sandboxing)
- **[SvelteKit](https://kit.svelte.dev/) + TypeScript** — lightweight, reactive frontend UI
- **[Vite](https://vitejs.dev/)** — frontend build tooling

---

## Features (Planned)

### Phase 1 — Core Launcher
- **Command-driven UI**: type multi-word phrases to trigger actions
- **Partial matching**: results appear as you type, showing possible completions as subtext
- **Open URL**: navigate to any URL in your default browser; supports an optional `param` query variable
- **Paste Text**: paste pre-defined text snippets into the previously focused application

### Phase 2 — Script Extensions
- Associate custom commands with external scripts or executables
- Scripts return plain text or structured JSON (title + subtext + action)
- Actions are fulfilled by the launcher's built-in functions — scripts cannot perform system actions themselves

---

## Using Ctx

| Guide | Description |
|-------|-------------|
| [First run](docs/using/first-run.md) | Onboarding walkthrough and choosing your global shortcut |
| [Basic functionality](docs/using/basic-functionality.md) | Open URL and Paste Text — the two built-in actions |
| [Tips & tricks](docs/using/tips-and-tricks.md) | App deep links, web search via param passing, frequently-pasted snippets |
| [Configuring commands](docs/using/configuring-commands.md) | YAML schema, enable/disable, directory layout, live reload |
| [Duplicate commands](docs/using/duplicate-commands.md) | How duplicate phrases are detected, resolved, and surfaced |

---

## Getting Started

For a full step-by-step guide see [docs/development-setup.md](docs/development-setup.md).

For the implementation roadmap see [docs/development-plan.md](docs/development-plan.md).

### Quick start

```bash
npm install
npm run tauri dev
```

### Build

```bash
npm run tauri build
```

---

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) with:
- [Svelte for VS Code](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

---

## Project Structure

```
bootstrap/     — Initial planning notes and project brief
docs/          — User and developer documentation
src/           — SvelteKit frontend source
src-tauri/     — Rust/Tauri backend source
static/        — Static frontend assets
plugins/       — Example and built-in plugin definitions (Phase 2+)
```

---

## Contributing

- Commit after every meaningful step with a descriptive message
- Raise questions rather than making assumptions when requirements are unclear
- Keep this README and the `docs/` folder up to date as the project evolves

---

## License

TBD

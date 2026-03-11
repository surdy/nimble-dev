# Ctx Launcher

A fast, cross-platform desktop launcher driven entirely by commands — think Alfred or Raycast, built around multi-word phrase commands with real-time partial matching and extensibility through sandboxed scripts.

---

## Tech Stack

- **[Tauri 2](https://tauri.app/)** — Rust-powered native shell (tiny binary, fast startup, strong sandboxing)
- **[SvelteKit](https://kit.svelte.dev/) + TypeScript** — lightweight, reactive frontend UI
- **[Vite](https://vitejs.dev/)** — frontend build tooling

---

## Features

### Built-in actions
- **Open URL** — navigate to any URL in your default browser; supports an optional `{param}` placeholder for user-supplied input
- **Paste Text** — paste pre-defined text snippets into the previously focused application via `⌘V` simulation
- **Copy Text** — copy pre-defined text to the clipboard without simulating a keystroke
- **Static List** — display a named YAML list of items inline as soon as the phrase is typed; selecting an item can paste, copy, or open a URL
- **Dynamic List** — run an external script and show its output as an inline list; supports `none`, `optional`, and `required` argument modes for real-time filtering

### Script extensions
- Associate commands with any executable (shell script, Python, Node.js, compiled binary, …)
- Scripts return plain text or a structured JSON array (`title` + optional `subtext`)
- All system actions flow through the launcher's built-in layer — scripts cannot directly modify the system

---

## Installation

### macOS

1. Download the latest `.dmg` from the [Releases page](https://github.com/your-org/contexts-launcher/releases)
2. Open the `.dmg`, drag **Ctx.app** into **Applications**, then eject the disk image

**Gatekeeper warning** — because Ctx is not yet notarized by Apple, macOS will block the first launch. To allow it:

1. Open **System Settings → Privacy & Security**
2. Scroll to the **Security** section and click **Open Anyway**
3. Confirm by clicking **Open** in the dialog

> **Alternative:** Right-click **Ctx.app** in Finder and choose **Open** — the dialog will include an **Open** button even for unsigned apps.

**Accessibility permission** — required for the Paste Text action to simulate `⌘V`:

1. Open **System Settings → Privacy & Security → Accessibility**
2. Click **+** and add **Ctx.app**, then enable its toggle

Without this, paste commands write to the clipboard but skip the keystroke simulation.

### Windows & Linux

Planned for a future release. Build from source in the meantime — see [docs/development-setup.md](docs/development-setup.md).

---

## Using Ctx

| Guide | Description |
|-------|-------------|
| [First run](docs/using/first-run.md) | Onboarding walkthrough and choosing your global shortcut |
| [Basic functionality](docs/using/basic/README.md) | Open URL, Paste Text, Copy Text |
| [Advanced features](docs/using/advanced/README.md) | Static List, Dynamic List, Script Extensions |
| [Tips & tricks](docs/using/tips-and-tricks.md) | App deep links, web search via param passing, frequently-pasted snippets |
| [Config directory structure](docs/using/config-directory.md) | Overview of the config directory layout and its subdirectories |
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

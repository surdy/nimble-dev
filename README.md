# Contexts Launcher

A fast, cross-platform desktop launcher driven entirely by commands — think Alfred or Raycast, but built around multi-word phrase commands with real-time partial matching and extensibility through sandboxed scripts.

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

## Getting Started

> **Note:** The project is in its initial planning phase. Setup and build instructions will be added here as development progresses.

---

## Project Structure

```
bootstrap/   — Initial planning notes and project brief
docs/        — User and developer documentation
src/         — Application source code
plugins/     — Example and built-in plugin definitions (Phase 2+)
```

---

## Contributing

- Commit after every meaningful step with a descriptive message
- Raise questions rather than making assumptions when requirements are unclear
- Keep this README and the `docs/` folder up to date as the project evolves

---

## License

TBD

# Decisions — Context Actions (Summary)

One-line record of every meaningful decision. See `decisions-details.md` for full rationale and risk analysis.

---

- **Linux focus tracking** — `xdotool` subprocess (consistent with `pbcopy` pattern, zero new Rust deps) _(2026-03-11)_
- **Cross-platform clipboard** — `pbcopy` on macOS retained; `arboard` crate added for Linux & Windows _(2026-03-11)_
- **Windows Win32 bindings** — `windows-sys` (Microsoft-published, minimal FFI, feature-gated) over `winapi` or `windows` _(2026-03-11)_
- **arboard crate scoping** — unconditional `[dependencies]` (simpler; per-platform cost is minimal) _(2026-03-11)_


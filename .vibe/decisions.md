# Decisions — Context Actions (Summary)

One-line record of every meaningful decision. See `decisions-details.md` for full rationale and risk analysis.

---

- **Linux focus tracking (reversal)** — switched from `xdotool` subprocess to `libxdo-sys` FFI; `xdotool` incompatible with Flatpak sandbox _(2026-03-12)_
- **Cross-platform clipboard** — `pbcopy` on macOS retained; `arboard` crate added for Linux & Windows _(2026-03-11)_
- **Windows Win32 bindings** — `windows-sys` (Microsoft-published, minimal FFI, feature-gated) over `winapi` or `windows` _(2026-03-11)_
- **arboard crate scoping** — unconditional `[dependencies]` (simpler; per-platform cost is minimal) _(2026-03-11)_
- **PreviousApp state type** — `String` (decimal ID) to unify macOS PID + Linux window ID + future Windows HWND _(2026-03-12)_
- **Linux focus capture timing** — capture before `window.show()` to avoid capturing our own window _(2026-03-12)_

- **Windows HWND storage type** — `isize` decimal string (matches `windows-sys` return type; zero guard handles invalid HWNDs) _(2026-03-13)_
- **Windows PowerShell invocation** — `powershell -ExecutionPolicy Bypass -File` on `.ps1` extension; `powershell.exe` chosen over `pwsh` for default availability _(2026-03-13)_
- **CI Linux packaging** — Flatpak (sandboxed, modern Linux standard; GNOME SDK 45 installed in CI) over AppImage or deb _(2026-03-12)_
- **CI Rust toolchain action** — `dtolnay/rust-toolchain@stable` (actively maintained, fast) over deprecated `actions-rs` _(2026-03-12)_

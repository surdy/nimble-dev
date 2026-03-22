# Decisions — Nimble (Summary)

One-line record of every meaningful decision. See `decisions-details.md` for full rationale and risk analysis.

---

- **Linux focus tracking** — `xdotool` subprocess (consistent with `pbcopy` pattern, zero new Rust deps) _(2026-03-11)_
- **Cross-platform clipboard** — `pbcopy` on macOS retained; `arboard` crate added for Linux & Windows _(2026-03-11)_
- **Windows Win32 bindings** — `windows-sys` (Microsoft-published, minimal FFI, feature-gated) over `winapi` or `windows` _(2026-03-11)_
- **arboard crate scoping** — unconditional `[dependencies]` (simpler; per-platform cost is minimal) _(2026-03-11)_
- **PreviousApp state type** — `String` (decimal ID) to unify macOS PID + Linux window ID + future Windows HWND _(2026-03-12)_
- **Linux focus capture timing** — capture before `window.show()` to avoid capturing our own window _(2026-03-12)_

- **Windows HWND storage type** — `isize` decimal string (matches `windows-sys` return type; zero guard handles invalid HWNDs) _(2026-03-13)_
- **Windows PowerShell invocation** — `powershell -ExecutionPolicy Bypass -File` on `.ps1` extension; `powershell.exe` chosen over `pwsh` for default availability _(2026-03-13)_
- **CI Linux packaging** — Flatpak (sandboxed, modern Linux standard; GNOME SDK 45 installed in CI) over AppImage or deb _(2026-03-12)_
- **CI Rust toolchain action** — `dtolnay/rust-toolchain@stable` (actively maintained, fast) over deprecated `actions-rs` _(2026-03-12)_
- **Linux focus tracking (reversal of: "Linux focus tracking" 2026-03-11)** — switched from `xdotool` subprocess to `libxdo-sys` FFI; `xdotool` incompatible with Flatpak sandbox _(2026-03-12)_

- **App renamed to Nimble** — product name, binary name, bundle identifier, config directory, and all documentation updated from "Context Actions" / "context-actions" / "ContextActions" to "Nimble" / "nimble" _(2025-07-14)_
- **User-defined env variable layering** — three layers (global env.yaml, sidecar env.yaml, inline env:) with deterministic override order; no directory walking for sidecar; NIMBLE_ prefix reserved _(2026-03-15)_
- **External paths default-on with opt-out** — `allow_external_paths: true` by default; `${VAR}` substitution in script/list fields; opt-out via settings.yaml for containment _(2025-07-15)_
- **Static list file format: TSV over YAML/CSV** — TSV chosen for human editability; no quoting rules, spreadsheet-paste friendly, commas in data just work; also eliminates YAML parse-error noise from co-located list files _(2026-03-21)_
- **Docs restructure: actions/guides/reference over using/basic/advanced** — replaced `using/` with `actions/` (all 6 action types), `guides/` (workflow docs), `reference/` (lookup material); eliminates skill-level split that mixed action types with feature docs _(2026-03-21)_
- **Private/public repo split with tag-triggered sync** — file-copy-and-squash via rsync in CI; one commit per release on public repo; excludes `.vibe/`, `bootstrap/`, `.github/agents/`, `.github/chatmodes/`, `.github/prompts/`, `.github/copilot-instructions.md`; PAT auth for cross-repo push _(2026-03-22)_

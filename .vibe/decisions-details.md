# Architecture & Technology Decisions — Context Actions

This file logs every meaningful trade-off and technology decision made during development.
Entries are appended chronologically. Each entry follows the structure below.

---

## Linux focus tracking: xdotool subprocess vs Rust xcb/x11rb crate
_Date: 2026-03-11_

### Options evaluated
**Option A — `xdotool` subprocess**
- Pros: zero new Rust crate dependencies; battle-tested tool already installed on most Linux desktops; trivial to call via `std::process::Command`; same pattern already used by `pbcopy` on macOS
- Cons: requires `xdotool` to be installed as a system package; subprocess overhead on each show/hide; does not work under pure Wayland (no XWayland bridge)

**Option B — `xcb` / `x11rb` Rust crate**
- Pros: no external binary dependency; pure Rust; can query `_NET_ACTIVE_WINDOW` directly from the Rust process; more reliable on unusual X11 setups
- Cons: adds a heavy, complex dependency (~20 transitive crates); requires significant boilerplate to establish a connection, query the property, and parse the EWMH atom; still does not help on Wayland

### Decision
Chosen Option A (`xdotool`). The `pbcopy` subprocess pattern is already established and proven in this codebase, keeping the approach consistent. The dependency on a system binary is acceptable — `xdotool` is available in every major Linux distro's package manager and is standard on X11 desktops. If pure-Wayland support becomes a priority, a separate Wayland-native approach (e.g. `wlr-foreign-toplevel-management`) should be scoped as its own stage rather than complicating this one.

### Risks & pitfalls
- `xdotool` may not be installed by default on minimal desktop environments; must document it as a runtime dependency
- `xdotool windowfocus` can fail silently if the window manager does not support `_NET_ACTIVE_WINDOW` (rare but possible with exotic WMs)
- Subprocess latency adds ~10–30 ms to the focus-restore path; this should be imperceptible but needs testing on slow machines
- Does not work under Wayland without XWayland; graceful degradation (skip focus restore, warn) is planned

---

## Cross-platform clipboard: arboard crate vs per-platform subprocesses
_Date: 2026-03-11_

### Options evaluated
**Option A — `arboard` Rust crate (unified)**
- Pros: single API for macOS, Linux (X11 + Wayland), and Windows; pure Rust; actively maintained; handles clipboard ownership lifetimes correctly
- Cons: on macOS, `NSPasteboard` has threading restrictions — `arboard` runs clipboard calls on a dedicated thread internally, which adds complexity and has historically had edge-case bugs on macOS; slightly larger binary

**Option B — `pbcopy` on macOS, `arboard` on Linux/Windows**
- Pros: keeps the proven `pbcopy` path on macOS untouched (zero risk of macOS regression); `arboard` only introduced where there is no working alternative; minimises blast radius of the change
- Cons: two different code paths to maintain; if `arboard` works reliably on macOS it would be unnecessary duplication

**Option C — platform subprocesses everywhere (`pbcopy`, `xclip`/`xsel`, PowerShell)**
- Pros: no new Rust crate at all
- Cons: `xclip`/`xsel` vary by distro and may not be installed; Windows subprocess clipboard is awkward; much more fragile overall

### Decision
Chosen Option B: keep `pbcopy` on macOS, add `arboard` for Linux and Windows. macOS clipboard behaviour is already working and tested; introducing `arboard` there would risk a regression with no benefit. On Linux and Windows there is currently no working implementation, so `arboard` is the cleanest path forward.

### Risks & pitfalls
- `arboard` on Linux requires a running display server (X11 or Wayland); it will panic or error in headless environments — clipboard integration tests must be gated with `#[ignore]`
- On Wayland, `arboard` clipboard content may be lost when the process that set it exits (Wayland clipboard model); for `paste_text` this is fine (the text is pasted immediately), but `copy_text` should document this limitation
- If `arboard` introduces a macOS regression in a future update it would be harder to diagnose because the macOS codepath uses `pbcopy` — the divergence is intentional but must be documented

---

## Windows Win32 crate: windows-sys vs winapi
_Date: 2026-03-11_

### Options evaluated
**Option A — `windows-sys` (Microsoft-published)**
- Pros: published and maintained by Microsoft; minimal, raw FFI bindings only (no smart wrappers adding overhead); feature-gated so only the required Win32 modules are compiled; recommended by the Rust Windows ecosystem going forward
- Cons: raw `unsafe` bindings with no ergonomic wrappers; more verbose call sites than higher-level crates

**Option B — `winapi` crate (community)**
- Pros: widely used in older Rust code; familiar to many Rust developers
- Cons: no longer actively maintained; Microsoft's own guidance now points to `windows` or `windows-sys`; larger surface area than needed

**Option C — `windows` crate (Microsoft, with safe wrappers)**
- Pros: safer, more ergonomic API; still Microsoft-maintained
- Cons: much larger compile-time footprint; safe wrappers add binary size and compile time that is not justified for two simple Win32 calls

### Decision
Chosen Option A (`windows-sys`). It is the minimal, Microsoft-maintained, actively developed binding for exactly this use case. Two Win32 calls (`GetForegroundWindow`, `SetForegroundWindow`) do not justify pulling in safe wrapper overhead from the `windows` crate.

### Risks & pitfalls
- Raw `unsafe` code must be carefully reviewed to avoid undefined behaviour from dangling HWNDs
- `SetForegroundWindow` is subject to Windows focus-stealing prevention rules; it may silently fail if the calling process is not the foreground process — the `HWND` should be stored and the call made immediately after the launcher hides, before Windows decides to block it
- Feature flags in `windows-sys` must be kept in sync if additional Win32 APIs are needed later

---

## arboard crate: unconditional vs macOS-excluded target dependency
_Date: 2026-03-11_

### Options evaluated
**Option A — Unconditional `[dependencies]` entry**
- Pros: simpler Cargo.toml; `arboard` compiles on all platforms regardless of use; no need for `#[cfg(not(target_os = "macos"))]` on the dependency
- Cons: adds unused compile overhead on macOS (arboard's macOS backend is compiled but never called); slightly larger macOS binary

**Option B — `[target.'cfg(not(target_os = "macos"))'.dependencies]`**
- Pros: arboard is only compiled on platforms where it is actually used; cleaner separation
- Cons: more complex Cargo.toml; if the macOS `pbcopy` path is ever replaced by `arboard`, the dependency entry must be moved

### Decision
Chosen Option A (unconditional). The compile cost on macOS is minimal and the simplicity of a single dependency entry outweighs it. The `arboard` crate documents that it gates its platform backends internally, so the macOS overhead is contained.

### Risks & pitfalls
- If `arboard` introduces a macOS build failure in a future version, it would break the macOS build even though it is not used there
- The unused macOS backend code slightly increases binary size on macOS; acceptable for now

---

## PreviousApp state type: i32 PID vs String for cross-platform IDs
_Date: 2026-03-12_

### Options evaluated
**Option A — Keep `i32`, add a second field for Linux window ID**
- Pros: macOS code unchanged; explicit typing per platform
- Cons: requires a custom enum or tuple struct; more boilerplate at every call site

**Option B — Change to `String`, store IDs as decimal strings**
- Pros: single unified state type; macOS PIDs and Linux X11 window IDs both fit; call sites are identical across platforms; trivial to extend to Windows HWND (also a numeric ID)
- Cons: macOS path now parses the string back to `i32`; marginal allocation overhead vs direct integer

### Decision
Chosen Option B (`String`). The uniformity across call sites outweighs the trivial parse cost. All platform ID types (PID, X11 window ID, HWND) are representable as decimal strings, making this approach future-proof for Windows too.

### Risks & pitfalls
- If `pid.to_string()` and `id.parse::<i32>()` ever diverge due to a bug, macOS focus restoration silently fails; the parse is infallible for valid PIDs so this risk is purely theoretical
- Variable name `prev_pid` at call sites is slightly misleading on Linux (it holds a window ID, not a PID); acceptable as a cosmetic issue

---

## Linux focus restore: capture timing (before vs after window show)
_Date: 2026-03-12_

### Options evaluated
**Option A — Capture before `window.show()`**
- Pros: launcher window is not yet visible, so `xdotool getactivewindow` reliably returns the user's app rather than our own window
- Cons: none apparent

**Option B — Capture after `window.show()`**
- Pros: none
- Cons: on X11, the launcher may have already received focus by the time `getactivewindow` is called, capturing ourselves instead of the target app

### Decision
Chosen Option A (capture before show). This mirrors the existing macOS pattern and avoids the race condition where the launcher steals focus before we can sample the active window.

### Risks & pitfalls
- On some X11 compositors there can be a short delay between `window.show()` and the window actually receiving focus; the capture-before-show approach is safe regardless
- If the hotkey triggers capture from the launcher window itself (hotkey pressed twice in rapid succession), we may capture our own window ID; the macOS guard `pid != std::process::id()` prevents this on macOS, but there is currently no equivalent guard on Linux — worst case: restore is a no-op

## Windows HWND type: isize vs u64 for PreviousApp storage
_Date: 2026-03-13_

### Options evaluated
**Option A — store as `isize` (HWND native type)**
- Pros: matches `windows-sys` API type; no cast needed at callsite
- Cons: signed integer is unusual for a handle; parse-back requires `isize::parse`

**Option B — store as `u64` string**
- Pros: clearly non-negative; portable width
- Cons: requires cast from `isize`; `isize` is always ≥ 0 for valid HWNDs

### Decision
Stored as decimal `isize` string to exactly match the `windows-sys` return type without casting. Valid HWNDs are always > 0 so sign is not an issue in practice; parse-back uses `id.parse::<isize>()`.

### Risks & pitfalls
- `isize` is 64-bit on 64-bit Windows; no truncation risk.
- A zero HWND correctly fails the `!= 0` guard in `capture_previous_app`.

---

## Windows PowerShell invocation strategy for .ps1 scripts
_Date: 2026-03-13_

### Options evaluated
**Option A — `powershell.exe` (Windows PowerShell 5.1, built-in)**
- Pros: present on every default Windows 10/11 installation; no user-side install required
- Cons: only ships Windows PowerShell 5.1; does not exist on non-Windows

**Option B — `pwsh.exe` (PowerShell 7+, cross-platform)**
- Pros: modern cross-platform PowerShell; richer language features and module ecosystem
- Cons: not installed by default on Windows; adds an extra install step for users

### Decision
`powershell.exe` chosen over `pwsh` because it ships on every default Windows installation. `.ps1` scripts are invoked as `powershell -ExecutionPolicy Bypass -File <path>` so users never need to change their machine-level execution policy.

### Risks & pitfalls
- `powershell.exe` may not be on PATH in heavily customised environments
- `-ExecutionPolicy Bypass` is process-scoped only — it does not change the machine policy
- Scripts containing `#Requires -RunAsAdministrator` will still fail; users must handle elevation themselves

## CI Linux packaging: Flatpak vs AppImage
_Date: 2026-03-12_

### Options evaluated
**Option A — Flatpak**
- Pros: sandboxed distribution; widely supported on modern Linux desktops; flathub is the standard install pathway; consistent runtime environment
- Cons: requires `flatpak-builder` + GNOME SDK installed in CI; longer setup step; app identifier must ideally follow reverse-DNS convention

**Option B — AppImage**
- Pros: single-file, no install needed; simpler CI (no runtime install)
- Cons: no sandbox; less aligned with modern Linux distribution practices; no central repository

**Option C — deb package**
- Pros: trivial CI setup; native on Ubuntu/Debian
- Cons: not cross-distro; requires maintainer infrastructure for a repository

### Decision
Flatpak chosen as the Linux distribution target. CI installs `flatpak-builder` and the GNOME SDK 45 runtime on the ubuntu-22.04 runner, then runs `tauri build --bundles flatpak`. Aligns with modern Linux packaging and the explicit goal in the Stage 26 plan.

### Risks & pitfalls
- GNOME SDK version pin (`//45`) will become outdated as new SDK releases ship; the CI workflow must be updated to stay in sync
- `flatpak-builder` install adds several minutes to CI; unavoidable for a Flatpak target
- Flatpak sandbox finish-args must correctly specify all required permissions or the app will malfunction at runtime
- `libxdo` must be added as a bundled module in the manifest before Flathub submission

---

## CI Rust toolchain action: dtolnay vs actions-rs
_Date: 2026-03-12_

### Options evaluated
**Option A — `dtolnay/rust-toolchain@stable` (chosen)**
- Pros: actively maintained; fast; community-standard replacement after `actions-rs` was deprecated
- Cons: third-party action (not GitHub-official)

**Option B — `actions-rs/toolchain@v1`**
- Pros: was the original canonical Rust CI action; widely documented in older tutorials
- Cons: unmaintained since 2022; known deprecation warnings on Node.js 16; repository archived

### Decision
`dtolnay/rust-toolchain@stable` used in the CI workflow.

### Risks & pitfalls
- None. `dtolnay/rust-toolchain` is widely used and kept up to date.

## Linux focus tracking: xdotool subprocess vs libxdo-sys crate
_Date: 2026-03-12_

### Options evaluated
**Option A — `xdotool` subprocess (original)**
- Pros: zero Rust deps; simple code
- Cons: fails inside Flatpak sandbox (host PATH not visible); requires runtime binary installed by user

**Option B — `libxdo-sys` FFI crate (chosen)**
- Pros: no runtime binary required; works inside Flatpak when libxdo is bundled; single build-time dep (`libxdo-dev`); exposes `xdo_get_active_window`, `xdo_focus_window`, `xdo_raise_window` directly
- Cons: raw `unsafe` FFI; slightly more code; `libxdo-dev` must be installed at compile time

**Option C — `xdg-desktop-portal` RemoteDesktop**
- Pros: universal Wayland support; no C dep
- Cons: no `get_active_window` equivalent (cannot capture focus); alarming permission prompt; input injection only

### Reversal of: "Linux focus tracking: xdotool subprocess vs Rust xcb/x11rb crate"
_Original decision:_ "Linux focus tracking: xdotool subprocess vs Rust xcb/x11rb crate" chose `xdotool` subprocess _(2026-03-11)_
_Trigger:_ User questioned whether `xdotool` would work inside a Flatpak sandbox; investigation confirmed the host PATH is not accessible at runtime inside the sandbox.
_Why we changed route:_ `libxdo-sys` links against `libxdo` at compile time so it can be bundled directly into the Flatpak and requires no runtime binary on the host. It also removes the user-facing system dependency on `xdotool` for non-Flatpak installs.

### Decision
Switched to `libxdo-sys` 0.11. Removes the `xdotool` runtime dependency, makes the Flatpak bundle self-contained, and uses the same underlying C library that `xdotool` itself wraps. The switch required less than 30 lines of change.

### Risks & pitfalls
- `libxdo-sys` requires `libxdo-dev` at compile time on Linux (CI installs it; documented in development-setup.md)
- The Flatpak manifest must include libxdo as a bundled module before publishing to Flathub
- X11 only — Wayland focus capture remains unsupported (same limitation as `xdotool`)

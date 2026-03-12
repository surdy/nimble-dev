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

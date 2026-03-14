use std::{path::PathBuf, sync::mpsc, thread, time::Duration};

use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::{AppHandle, Emitter};

use crate::commands;

/// The Tauri event name emitted to the frontend when commands are reloaded.
pub const COMMANDS_RELOADED_EVENT: &str = "commands://reloaded";

/// Content of the seed `hello.sh` script written on first launch (macOS / Linux).
const HELLO_SH: &str = r#"#!/bin/sh
# Example dynamic list script.
# Output a JSON array of objects with "title" and optional "subtext" fields,
# or plain text for a single-item result. Edit this file to return your own items.
echo '[{"title":"Hello from a script","subtext":"Edit scripts/hello.sh to customise"},{"title":"Dynamic lists are powerful","subtext":"Return JSON or plain text from any executable"}]'
"#;

/// Content of the seed `hello.ps1` script written on first launch (Windows).
const HELLO_PS1: &str = r#"# Example dynamic list script.
# Output a JSON array of objects with 'title' and optional 'subtext' fields,
# or plain text for a single-item result. Edit this file to return your own items.
Write-Output '[{"title":"Hello from a script","subtext":"Edit scripts/hello.ps1 to customise"},{"title":"Dynamic lists are powerful","subtext":"Return JSON or plain text from any executable"}]'
"#;

/// Start a background thread that watches `commands_dir` and the sibling
/// `scripts/` directory for file changes. On any relevant event the command
/// list is reloaded and emitted to all windows as `commands://reloaded`.
///
/// `commands_dir` is `config_dir/commands/`; the function derives
/// `config_dir/scripts/` automatically.
///
/// The watcher runs for the lifetime of the app — Tauri will clean up the
/// thread when the process exits.
pub fn start(app: AppHandle, commands_dir: PathBuf, allow_duplicates: bool) {
    thread::spawn(move || {
        // Channel for raw notify events
        let (tx, rx) = mpsc::channel();

        let mut watcher = match RecommendedWatcher::new(
            move |res| {
                if let Ok(event) = res {
                    let _ = tx.send(event);
                }
            },
            Config::default().with_poll_interval(Duration::from_secs(1)),
        ) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("[ctx] could not create file watcher: {e}");
                return;
            }
        };

        if let Err(e) = watcher.watch(&commands_dir, RecursiveMode::Recursive) {
            eprintln!("[ctx] could not watch commands dir: {e}");
            return;
        }

        // Also watch the sibling scripts/ directory so edits to scripts
        // trigger a commands://reloaded event (the frontend will re-invoke
        // run_dynamic_list if a dynamic list is currently displayed).
        let scripts_dir = commands_dir
            .parent()
            .map(|p| p.join("scripts"))
            .unwrap_or_else(|| commands_dir.join("../scripts"));
        // Ensure the scripts directory exists so the watcher can be registered.
        let _ = std::fs::create_dir_all(&scripts_dir);
        // Seed hello.sh on first launch on macOS / Linux (when it does not yet exist).
        #[cfg(unix)]
        {
            let hello_sh = scripts_dir.join("hello.sh");
            if !hello_sh.exists() {
                if let Ok(()) = std::fs::write(&hello_sh, HELLO_SH) {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(meta) = std::fs::metadata(&hello_sh) {
                        let mut perms = meta.permissions();
                        perms.set_mode(0o755);
                        let _ = std::fs::set_permissions(&hello_sh, perms);
                    }
                }
            }
        }
        // Seed hello.ps1 on first launch on Windows (when it does not yet exist).
        #[cfg(windows)]
        {
            let hello_ps1 = scripts_dir.join("hello.ps1");
            if !hello_ps1.exists() {
                let _ = std::fs::write(&hello_ps1, HELLO_PS1);
            }
        }
        if let Err(e) = watcher.watch(&scripts_dir, RecursiveMode::Recursive) {
            eprintln!("[ctx] could not watch scripts dir: {e}");
        }

        eprintln!(
            "[ctx] watching for changes: {}",
            commands_dir.display()
        );

        // Debounce: after a relevant event, wait this long before reloading
        // so that rapid saves / multiple renames don't trigger multiple loads.
        const DEBOUNCE: Duration = Duration::from_millis(300);

        loop {
            // Block until first event
            let event = match rx.recv() {
                Ok(e) => e,
                Err(_) => break, // channel closed, app is exiting
            };

            if !is_yaml_event(&event) {
                continue;
            }

            // Drain any additional events that arrive within the debounce window
            loop {
                match rx.recv_timeout(DEBOUNCE) {
                    Ok(_) => continue, // discard, keep draining
                    Err(mpsc::RecvTimeoutError::Timeout) => break,
                    Err(mpsc::RecvTimeoutError::Disconnected) => return,
                }
            }

            // Reload commands and emit to frontend
            match commands::load_from_dir(&commands_dir, allow_duplicates) {
                Ok(result) => {
                    if let Err(e) = app.emit(COMMANDS_RELOADED_EVENT, &result) {
                        eprintln!("[ctx] could not emit reload event: {e}");
                    }
                }
                Err(e) => {
                    eprintln!("[ctx] reload failed: {e}");
                }
            }
        }
    });
}

/// Returns true if the event is one we should react to:
/// - Create / Modify: only when a .yaml or .yml file is affected.
/// - Remove: any removal inside the watched config dir. On macOS, FSEvents can
///   report a deletion with the *parent directory path* rather than the deleted
///   file's path, so the extension check would incorrectly filter it out.
///   The config dir is low-churn, so reloading on any removal is harmless.
fn is_yaml_event(event: &notify::Event) -> bool {
    match event.kind {
        EventKind::Create(_) | EventKind::Modify(_) => {
            event.paths.iter().any(|p| {
                matches!(
                    p.extension().and_then(|e| e.to_str()),
                    Some("yaml") | Some("yml")
                )
            })
        }
        EventKind::Remove(_) => !event.paths.is_empty(),
        _ => false,
    }
}

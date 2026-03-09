use std::{path::PathBuf, sync::mpsc, thread, time::Duration};

use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::{AppHandle, Emitter};

use crate::commands;

/// The Tauri event name emitted to the frontend when commands are reloaded.
pub const COMMANDS_RELOADED_EVENT: &str = "commands://reloaded";

/// Start a background thread that watches `config_dir` recursively for YAML
/// file changes. On any relevant event the command list is reloaded and
/// emitted to all windows as `commands://reloaded`.
///
/// The watcher runs for the lifetime of the app — Tauri will clean up the
/// thread when the process exits.
pub fn start(app: AppHandle, config_dir: PathBuf) {
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
                eprintln!("[contexts] could not create file watcher: {e}");
                return;
            }
        };

        if let Err(e) = watcher.watch(&config_dir, RecursiveMode::Recursive) {
            eprintln!("[contexts] could not watch config dir: {e}");
            return;
        }

        eprintln!(
            "[contexts] watching config dir for changes: {}",
            config_dir.display()
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
            match commands::load_from_dir(&config_dir) {
                Ok(cmds) => {
                    if let Err(e) = app.emit(COMMANDS_RELOADED_EVENT, &cmds) {
                        eprintln!("[contexts] could not emit reload event: {e}");
                    }
                }
                Err(e) => {
                    eprintln!("[contexts] reload failed: {e}");
                }
            }
        }
    });
}

/// Returns true if the event involves a .yaml or .yml file and is a kind we
/// care about (create, modify, remove). Renames count as a remove + create pair
/// which notify surfaces individually.
fn is_yaml_event(event: &notify::Event) -> bool {
    matches!(
        event.kind,
        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
    ) && event.paths.iter().any(|p| {
        matches!(
            p.extension().and_then(|e| e.to_str()),
            Some("yaml") | Some("yml")
        )
    })
}

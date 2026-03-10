mod commands;
mod watcher;

use std::sync::{Arc, Mutex};
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
use tauri_plugin_opener::OpenerExt;

// ── Previous-app focus tracking ────────────────────────────────────────────────

/// Holds the PID of the application that had focus before the launcher appeared.
struct PreviousApp(Mutex<Option<i32>>);

/// Record the frontmost application's PID before we show the launcher.
/// Called in the global-shortcut handler and tray show/hide, so macOS knows
/// where to return focus when paste_text is executed.
#[cfg(target_os = "macos")]
fn capture_previous_app(state: &PreviousApp) {
    use objc2_app_kit::NSWorkspace;
    let workspace = NSWorkspace::sharedWorkspace();
    if let Some(app) = workspace.frontmostApplication() {
        let pid = app.processIdentifier();
        // Don't record ourselves
        if pid != std::process::id() as i32 {
            *state.0.lock().unwrap() = Some(pid);
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn capture_previous_app(_state: &PreviousApp) {}

/// Activate the app identified by `pid` so it regains keyboard focus.
#[cfg(target_os = "macos")]
fn restore_previous_app(pid: i32) {
    use objc2_app_kit::{NSApplicationActivationOptions, NSRunningApplication};
    if let Some(app) =
        NSRunningApplication::runningApplicationWithProcessIdentifier(pid)
    {
        // ActivateIgnoringOtherApps is deprecated in macOS 14 but still works;
        // it has no replacement on NSRunningApplication in objc2-app-kit 0.3.
        #[allow(deprecated)]
        app.activateWithOptions(
            NSApplicationActivationOptions::ActivateIgnoringOtherApps,
        );
    }
}

#[cfg(not(target_os = "macos"))]
fn restore_previous_app(_pid: i32) {}

// ── Clipboard helper ───────────────────────────────────────────────────────────

/// Write `text` to the system clipboard.
/// macOS: delegates to the `pbcopy` subprocess (no threading constraints).
/// Other platforms: uses `arboard` (add the crate to Cargo.toml when targeting
///                  Linux / Windows).
fn write_clipboard_text(text: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use std::io::Write;
        let mut child = std::process::Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Could not start pbcopy: {e}"))?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(text.as_bytes())
                .map_err(|e| format!("Could not write to pbcopy: {e}"))?;
        }
        child
            .wait()
            .map_err(|e| format!("pbcopy failed: {e}"))?;
        Ok(())
    }
    #[cfg(not(target_os = "macos"))]
    {
        // arboard is intentionally not yet in Cargo.toml for the macOS-only
        // build; add it when targeting Linux / Windows.
        Err("paste_text is not yet supported on this platform".to_string())
    }
}

/// Open a URL in the default browser or the registered handler for its scheme.
/// - Any well-formed URL scheme is accepted (e.g. http://, https://, slack://,
///   obsidian://, mailto:, …). Schemes must start with a letter and contain
///   only letters, digits, '+', '-', or '.' before the ':' — per RFC 3986.
///   Plain strings with no scheme at all are rejected.
/// - Occurrences of `{param}` in the URL are replaced with `param`
///   (URL-encoded) before opening.
#[tauri::command]
fn open_url(app: tauri::AppHandle, url: String, param: Option<String>) -> Result<(), String> {
    // Substitute {param} if present
    let resolved = if let Some(p) = param {
        let encoded: String = p
            .bytes()
            .flat_map(|b| match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9'
                | b'-' | b'_' | b'.' | b'~' => vec![b as char],
                b' ' => vec!['+'],
                _ => format!("%{:02X}", b).chars().collect(),
            })
            .collect();
        url.replace("{param}", &encoded)
    } else {
        url
    };

    // Validate scheme — must match RFC 3986: letter *( letter / digit / "+" / "-" / "." ) ":"
    // This accepts http://, https://, slack://, obsidian://, mailto:, etc.
    // and rejects bare strings, relative paths, and malformed schemes.
    let has_valid_scheme = resolved
        .find(':')
        .map(|colon| {
            let scheme = &resolved[..colon];
            !scheme.is_empty()
                && scheme.starts_with(|c: char| c.is_ascii_alphabetic())
                && scheme
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.')
        })
        .unwrap_or(false);

    if !has_valid_scheme {
        return Err(format!("Rejected URL with missing or invalid scheme: {resolved}"));
    }

    app.opener()
        .open_url(resolved, None::<&str>)
        .map_err(|e| e.to_string())
}

struct TrayMenuState {
    show_hide_item: Arc<MenuItem<tauri::Wry>>,
}

/// Update the tray Show/Hide item text to reflect current window visibility.
fn sync_tray(app: &tauri::AppHandle, visible: bool) {
    let text = if visible { "Hide" } else { "Show" };
    // Clone the Arc inside a block so `state` (and its borrow) is dropped before
    // we call set_text, avoiding any lifetime entanglement with State<'_,...>.
    let item = {
        let state = app.state::<TrayMenuState>();
        Arc::clone(&state.show_hide_item)
    };
    item.set_text(text).ok();
}

/// Hide the launcher window (keeps app running in background).
#[tauri::command]
fn hide_window(app: tauri::AppHandle, window: tauri::Window) {
    window.hide().ok();
    sync_tray(&app, false);
}

/// Show and focus the launcher window.
///
/// Resets to the base 640×64 size and re-centers on the active monitor before
/// making the window visible. This corrects drift caused by macOS anchoring
/// `setSize` calls from the bottom-left while the window is hidden: shrinking
/// from a tall (results-visible) state moves the window top downward, so we
/// must correct the position on every show.
#[tauri::command]
fn show_window(app: tauri::AppHandle, window: tauri::Window) {
    window.set_size(tauri::LogicalSize::new(640_f64, 64_f64)).ok();
    window.center().ok();
    window.show().ok();
    window.set_focus().ok();
    sync_tray(&app, true);
}

/// Return the full list of commands loaded from the user config directory,
/// along with any duplicate warnings detected during loading.
#[tauri::command]
fn list_commands(app: tauri::AppHandle) -> Result<commands::LoadResult, String> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|e| e.to_string())?;
    commands::load_from_dir(&config_dir)
}

/// Dismiss the launcher intentionally (Escape key, hotkey while visible, tray Hide).
/// Hides the window, updates the tray, and restores focus to the previously
/// active application. Distinct from `hide_window` which is used for blur
/// dismissal where the OS already transferred focus to the new frontmost app.
#[tauri::command]
fn dismiss_launcher(app: tauri::AppHandle, window: tauri::Window) {
    window.hide().ok();
    sync_tray(&app, false);
    let prev_pid = app.state::<PreviousApp>().0.lock().unwrap().take();
    if let Some(pid) = prev_pid {
        restore_previous_app(pid);
    }
}

/// Register (or replace) the global hotkey that summons the launcher.
#[tauri::command]
fn register_shortcut(app: tauri::AppHandle, shortcut: String) -> Result<(), String> {
    app.global_shortcut()
        .unregister_all()
        .map_err(|e| e.to_string())?;

    let shortcut_str = shortcut.clone();
    app.global_shortcut()
        .on_shortcut(shortcut_str.as_str(), move |app, _shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        window.hide().ok();
                        sync_tray(app, false);
                        // Restore focus to the app that was active before we appear
                        let prev_pid = app.state::<PreviousApp>().0.lock().unwrap().take();
                        if let Some(pid) = prev_pid {
                            restore_previous_app(pid);
                        }
                    } else {
                        // Capture the frontmost app before we steal focus
                        let prev = app.state::<PreviousApp>();
                        capture_previous_app(&prev);
                        window.set_size(tauri::LogicalSize::new(640_f64, 64_f64)).ok();
                        window.center().ok();
                        window.show().ok();
                        window.set_focus().ok();
                        sync_tray(app, true);
                    }
                }
            }
        })
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Paste pre-defined plain text into the application that had focus before
/// the launcher was invoked.
///
/// Flow:
///   1. Validate the text (plain text only; reject NUL bytes).
///   2. Hide the launcher window and update the tray label.
///   3. Restore focus to the previously active application.
///   4. Write the text to the clipboard.
///   5. Simulate Cmd+V (macOS) / Ctrl+V (other) to trigger a paste.
///
/// Requires macOS Accessibility permission for the key-simulation step.
#[tauri::command]
fn paste_text(app: tauri::AppHandle, window: tauri::Window, text: String) -> Result<(), String> {
    // Sanitise: plain text only — reject NUL bytes
    if text.contains('\0') {
        return Err("Text must not contain NUL bytes".to_string());
    }

    // 1. Hide launcher
    window.hide().ok();
    sync_tray(&app, false);

    // 2. Restore focus to the previous app
    let prev_pid = app.state::<PreviousApp>().0.lock().unwrap().take();
    if let Some(pid) = prev_pid {
        restore_previous_app(pid);
    }

    // Brief pause so focus transfer completes before we write to the clipboard
    // and simulate the keystroke.
    std::thread::sleep(std::time::Duration::from_millis(80));

    // 3. Write text to clipboard
    write_clipboard_text(&text)?;

    // 4. Simulate paste keystroke
    use enigo::{Direction, Enigo, Key, Keyboard, Settings};
    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;

    #[cfg(target_os = "macos")]
    {
        enigo.key(Key::Meta, Direction::Press).map_err(|e| e.to_string())?;
        enigo.key(Key::Unicode('v'), Direction::Click).map_err(|e| e.to_string())?;
        enigo.key(Key::Meta, Direction::Release).map_err(|e| e.to_string())?;
    }
    #[cfg(not(target_os = "macos"))]
    {
        enigo.key(Key::Control, Direction::Press).map_err(|e| e.to_string())?;
        enigo.key(Key::Unicode('v'), Direction::Click).map_err(|e| e.to_string())?;
        enigo.key(Key::Control, Direction::Release).map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// Copy text to the clipboard without pasting it.
/// The launcher is hidden and the text is written to the clipboard;
/// no focus restoration or keystroke simulation is performed.
#[tauri::command]
fn copy_text(window: tauri::Window, app: tauri::AppHandle, text: String) -> Result<(), String> {
    // Sanitise: plain text only — reject NUL bytes
    if text.contains('\0') {
        return Err("Text must not contain NUL bytes".to_string());
    }

    window.hide().ok();
    sync_tray(&app, false);

    write_clipboard_text(&text)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // macOS: hide from Dock and Cmd+Tab app switcher
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // Build system tray menu
            let version = app.package_info().version.to_string();
            let app_info = MenuItem::new(
                app,
                format!("Ctx v{}", version),
                false,
                None::<&str>,
            )?;
            let sep1 = PredefinedMenuItem::separator(app)?;
            let show_hide = MenuItem::with_id(app, "show_hide", "Show", true, None::<&str>)?;
            let sep2 = PredefinedMenuItem::separator(app)?;
            let quit = MenuItem::with_id(app, "quit", "Quit Ctx", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&app_info, &sep1, &show_hide, &sep2, &quit])?;

            // Manage state for updating the Show/Hide item label
            app.manage(TrayMenuState {
                show_hide_item: Arc::new(show_hide),
            });

            // Manage previous-app tracking for paste_text focus restoration
            app.manage(PreviousApp(Mutex::new(None)));

            // Start watching the config directory for live command reloads
            let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
            watcher::start(app.handle().clone(), config_dir);

            let icon = app
                .default_window_icon()
                .cloned()
                .expect("no default window icon configured");

            TrayIconBuilder::new()
                .icon(icon)
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show_hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                window.hide().ok();
                                sync_tray(app, false);
                                // Restore focus to the app that was active before we appeared
                                let prev_pid = app.state::<PreviousApp>().0.lock().unwrap().take();
                                if let Some(pid) = prev_pid {
                                    restore_previous_app(pid);
                                }
                            } else {
                                // Capture previous app before we steal focus
                                let prev = app.state::<PreviousApp>();
                                capture_previous_app(&prev);
                                window.set_size(tauri::LogicalSize::new(640_f64, 64_f64)).ok();
                                window.center().ok();
                                window.show().ok();
                                window.set_focus().ok();
                                sync_tray(app, true);
                            }
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![hide_window, show_window, dismiss_launcher, register_shortcut, list_commands, open_url, paste_text, copy_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

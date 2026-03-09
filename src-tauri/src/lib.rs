mod commands;

use std::sync::Arc;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

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
#[tauri::command]
fn show_window(app: tauri::AppHandle, window: tauri::Window) {
    window.show().ok();
    window.set_focus().ok();
    sync_tray(&app, true);
}

/// Return the full list of commands loaded from the user config directory.
#[tauri::command]
fn list_commands(app: tauri::AppHandle) -> Result<Vec<commands::Command>, String> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|e| e.to_string())?;
    commands::load_from_dir(&config_dir)
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
                    } else {
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
                format!("Contexts v{}", version),
                false,
                None::<&str>,
            )?;
            let sep1 = PredefinedMenuItem::separator(app)?;
            let show_hide = MenuItem::with_id(app, "show_hide", "Show", true, None::<&str>)?;
            let sep2 = PredefinedMenuItem::separator(app)?;
            let quit = MenuItem::with_id(app, "quit", "Quit Contexts", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&app_info, &sep1, &show_hide, &sep2, &quit])?;

            // Manage state for updating the Show/Hide item label
            app.manage(TrayMenuState {
                show_hide_item: Arc::new(show_hide),
            });

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
                            } else {
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
        .invoke_handler(tauri::generate_handler![hide_window, show_window, register_shortcut, list_commands])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

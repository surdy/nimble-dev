use tauri::Manager;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

/// Hide the launcher window (keeps app running in background)
#[tauri::command]
fn hide_window(window: tauri::Window) {
    window.hide().ok();
}

/// Show and focus the launcher window
#[tauri::command]
fn show_window(window: tauri::Window) {
    window.show().ok();
    window.set_focus().ok();
}

/// Register (or replace) the global hotkey that summons the launcher
#[tauri::command]
fn register_shortcut(app: tauri::AppHandle, shortcut: String) -> Result<(), String> {
    // Unregister all previous shortcuts before registering the new one
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
                    } else {
                        window.show().ok();
                        window.set_focus().ok();
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
        .invoke_handler(tauri::generate_handler![hide_window, show_window, register_shortcut])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

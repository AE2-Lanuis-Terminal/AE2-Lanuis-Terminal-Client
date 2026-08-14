//! AE2 Lanuis Client（Tauri 2）库入口：Windows 桌面 + Android。

#[cfg(desktop)]
mod settings_window;
#[cfg(desktop)]
mod tray;
#[cfg(desktop)]
mod window_api;

use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionConfig {
    pub server_host: String,
    pub server_port: u16,
    pub protocol: String,
    pub account: String,
}

#[tauri::command]
fn get_connection_config(app: tauri::AppHandle) -> Result<ConnectionConfig, String> {
    let dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let path = dir.join("connection.json");
    if !path.exists() {
        return Ok(ConnectionConfig {
            server_host: "127.0.0.1".into(),
            server_port: 8765,
            protocol: "http".into(),
            account: String::new(),
        });
    }
    let raw = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&raw).map_err(|e| e.to_string())
}

#[tauri::command]
fn set_connection_config(app: tauri::AppHandle, config: ConnectionConfig) -> Result<(), String> {
    let dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("connection.json");
    let raw = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    std::fs::write(path, raw).map_err(|e| e.to_string())
}

#[cfg(desktop)]
fn run_desktop() {
    use tauri::{Emitter, WindowEvent};
    use window_api::MAIN_LABEL;

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            tray::create_tray(app.handle())?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == settings_window::SETTINGS_LABEL {
                    return;
                }
                if window.label() == MAIN_LABEL {
                    api.prevent_close();
                    let _ = window.emit("window-close-requested", ());
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_connection_config,
            set_connection_config,
            tray::show_main_window_cmd,
            tray::hide_main_window_cmd,
            tray::exit_app,
            tray::sync_tray_menu_labels,
            settings_window::open_settings_window,
            settings_window::close_settings_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(mobile)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn run_mobile() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            get_connection_config,
            set_connection_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(desktop)]
pub fn run() {
    run_desktop();
}

#[cfg(mobile)]
pub fn run() {
    run_mobile();
}

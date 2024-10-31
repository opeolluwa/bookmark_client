#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub mod app_state;
pub mod commands;
pub mod config;
pub mod jwt;
pub mod api;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

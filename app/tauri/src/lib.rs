#[cfg_attr(mobile, tauri::mobile_entry_point)]

pub mod commands;

use commands::authentication;

pub fn run() {
    tauri::Builder::default()
        // .plugin(tauri_plugin_updater::Builder::new().build())
        // .plugin(tauri_plugin_single_instance::init())
        // .plugin(tauri_plugin_stronghold::Builder::new(|pass| todo!()).build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![authentication::sign_up])
        // .plugin(tauri_plugin_autostart::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

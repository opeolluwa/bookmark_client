pub mod api;
pub mod app_state;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub mod commands;
pub mod config;

use commands::authentication;

pub fn run() {
    tauri::Builder::default()
        // .setup(|app| {
        //     app.manage(AppState::from(db));
        //     Ok(())
        // })
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![authentication::sign_up])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use models::application_settings::AppSettings;
use sqlite_wasm_bindgen::database as sqlite_wasm_bindgen_database;
use tauri::Manager;

pub mod api_request;
pub mod commands;
pub mod database;
pub mod ipc_response;
pub mod models;

use crate::commands::application_settings;
use crate::commands::authentication;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let connection = sqlite_wasm_bindgen_database::SqliteWasm::init();
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            authentication::sign_up,
            application_settings::fetch_default_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

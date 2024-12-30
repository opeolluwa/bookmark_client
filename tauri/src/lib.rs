pub mod api_request;
pub mod commands;
pub mod database;
pub mod ipc_response;
pub mod models;
pub mod state;

#[allow(dead_code, unused_assignments)]
use std::sync::Mutex;

use tauri::Manager;

use sqlite_wasm_bindgen::database as sqlite_wasm_bindgen_database;

use crate::commands::authentication;
use crate::commands::settings;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let database_path = "test.db";
            let connection = sqlite_wasm_bindgen_database::SqliteWasm::init(&database_path)
                .expect("error initializing database");

            app.manage(state::AppState {
                conn: Mutex::new(connection),
            });
            Ok(())
        })
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            authentication::sign_up,
            settings::fetch_default_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub mod api_request;
pub mod commands;
pub mod ipc_response;
pub mod state;

use std::fs::create_dir_all;
#[allow(dead_code, unused_assignments)]
use std::sync::Mutex;

use tauri::Manager;
use tauri_plugin_fs::FsExt;

use bookmark_local_database::database::SqliteWasm;

use crate::commands::authentication;
use crate::commands::settings;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let scope = app.fs_scope();
            scope
                .allow_directory("/databases", true)
                .unwrap_or_else(|err| {
                    eprintln!("Error allowing directory: {err}");
                });
            scope
                .allow_file("/databases/bookmark.dev.sqlite")
                .unwrap_or_else(|err| {
                    eprintln!("Error allowing file: {err}");
                });

            // Define the embedded SQLite path
            let embedded_sqlite_path = app
                .path()
                .app_data_dir()
                .expect("Failed to fetch app data directory")
                .join("databases");

            // Create the directory if it doesn't exist
            create_dir_all(&embedded_sqlite_path).expect("Failed to create database directory");

            // Create the SQLite file if it doesn't exist
            let sqlite_file_path = embedded_sqlite_path.join("bookmark.dev.sqlite");
            if !sqlite_file_path.exists() {
                std::fs::File::create(&sqlite_file_path).expect("Failed to create SQLite file");
            }

            // Convert the file path to a string
            let database_path = sqlite_file_path
                .to_str()
                .expect("Failed to convert database path to string");

            // Initialize the SQLite connection
            let connection =
                SqliteWasm::init(database_path).expect("Failed to initialize SQLite database");

            // Manage application state
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

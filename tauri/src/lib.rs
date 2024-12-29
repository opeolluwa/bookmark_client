#[allow(dead_code, unused_assignments)]
use std::env::var;
use std::sync::Mutex;

use sqlite_wasm_bindgen::database as sqlite_wasm_bindgen_database;
use tauri::path::BaseDirectory;
use tauri::Manager;

pub mod api_request;
pub mod commands;
pub mod database;
pub mod ipc_response;
pub mod models;
pub mod state;

use crate::commands::authentication;
use crate::commands::settings;

// #[allow(dead_code, unused_assignments)]

// static DEVELOPMENT_ENVIRONMENT: &'static str = "development";
// static NONE_DEVELOPMENT_ENVIRONMENT: &'static str = "production";
// static DEVELOPMENT_DATABASE_FILE_PATH: &'static str = "resources/bookmark.dev.sqlite";
// static NON_DEVELOPMENT_DATABASE_FILE_PATH: &'static str = "resources/bookmark.sqlite";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // let application_run_environment =
            //     var("ENV").unwrap_or(NONE_DEVELOPMENT_ENVIRONMENT.to_string());

            // let database_file_path =
            //     if application_run_environment.as_str() == DEVELOPMENT_ENVIRONMENT {
            //         DEVELOPMENT_DATABASE_FILE_PATH
            //     } else {
            //         NON_DEVELOPMENT_DATABASE_FILE_PATH
            //     };


            let database_path = app
                .path()
                .resolve("sqlite/bookmark.dev.sqlite", BaseDirectory::Resource)?;

            let database_path = database_path.to_str().unwrap().to_string();

            println!("Database path: {}", database_path);
            // let path = std::fs::File::open(&database_path).unwrap();

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

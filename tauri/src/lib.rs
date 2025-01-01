pub mod api_request;
pub mod commands;
pub mod database;
pub mod ipc_response;
pub mod models;
pub mod state;

use std::fs::create_dir_all;
#[allow(dead_code, unused_assignments)]
use std::sync::Mutex;

use bookmark_local_database::database::SqliteWasm;
use tauri::Manager;
use tauri_plugin_fs::FsExt;

use crate::commands::authentication;
use crate::commands::settings;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let scope = app.fs_scope();
            let _ = scope.allow_directory("/databases", true);
            let _ = scope.allow_file("/databases/bookmark.dev.sqlite");

            let embedded_sqlite_path = app
                .path()
                .app_data_dir()
                .expect("error fetching app data dir")
                .join("databases");

            // create the path if not exist
            let _ = create_dir_all(&embedded_sqlite_path).expect("error creating path");
            let _ = std::fs::File::create(embedded_sqlite_path.join("bookmark.dev.sqlite"))
                .expect("error creating sqlite file ");

            // let embedded_sqlite_path = file.

            let binding = embedded_sqlite_path.join("bookmark.dev.sqlite");
            let database_path = binding.to_str().expect("error creating path");

            // let database_path = "test.db";
            let connection = SqliteWasm::init(database_path).expect("error initializing database");

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

use models::application_settings::AppSettings;
use tauri::Manager;

pub mod commands;
pub mod database;
pub mod models;

use crate::commands::application_settings;
use crate::commands::authentication;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db_instance = database::BookmarksDatabaseWasm::init().unwrap();

    tauri::Builder::default()
        // .plugin(tauri_plugin_http::init())
        .setup(|app| {
            let read_write_transaction = db_instance
                .rw_transaction()
                .expect("failed to create read_write_transaction migration transaction");
            read_write_transaction
                .migrate::<AppSettings>()
                .expect("failed to migrate Person");
            read_write_transaction
                .commit()
                .expect("failed to commit migration");

            app.manage(db_instance);
            Ok(())
        })
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            authentication::sign_up,
            application_settings::fetch_default_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use models::application_settings::AppSettings;
use native_db::{Database, Models};
use once_cell::sync::Lazy;
use tauri::Manager;

pub mod commands;
pub mod database;
pub mod models;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // let _ = sqlite_wasm::database::SqliteWasm::init();
    let db = database::BookmarksDatabaseWasm::init().unwrap();

    tauri::Builder::default()
        .setup(|app| {
            let rw = db
                .rw_transaction()
                .expect("failed to create rw migration transaction");
            rw.migrate::<AppSettings>()
                .expect("failed to migrate Person");
            rw.commit().expect("failed to commit migration");

            app.manage(db);
            Ok(())
        })
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

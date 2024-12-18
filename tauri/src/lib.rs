use models::application_settings::AppSettings;
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
    let db_instance = database::BookmarksDatabaseWasm::init().unwrap();

    tauri::Builder::default()
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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

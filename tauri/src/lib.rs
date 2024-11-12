use lazy_static::lazy_static;
use tauri_plugin_store::StoreExt;
lazy_static! {
    pub static ref GRPC_SERVER_ENDPOINT: String = String::from("http://127.0.0.1:50051");
}

pub mod app_state;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub mod commands;
pub mod config;
pub mod helpers;
pub fn run() {
    // let app_state = JsonDb::new::<TokenStore>("token_store").unwrap();

    tauri::Builder::default()
        .setup(|app| {
            let _ = app.store("store.json")?;
            // app.manage(app_state);
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            commands::authentication::sign_in,
            commands::bookmarks::get_all_bookmark_collections,
            commands::bookmarks::create_bookmark_collection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

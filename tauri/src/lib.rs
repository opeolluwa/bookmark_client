use lazy_static::lazy_static;
use tauri_plugin_store::StoreExt;

lazy_static! {
    pub static ref GRPC_SERVER_ENDPOINT: String = String::from("http://127.0.0.1:50051");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub mod commands;

pub mod helpers;
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let store = app.store("store.json")?;
            Ok(())
        })
        // .plugin(
        //     tauri_plugin_stronghold::Builder::new(|password| {
        //         // Hash the password here with e.g. argon2, blake2b or any other secure algorithm
        //         // Here is an example implementation using the `rust-argon2` crate for hashing the password
        //         use argon2::{hash_raw, Config, Variant, Version};

        //         let config = Config {
        //             lanes: 4,
        //             mem_cost: 10_000,
        //             time_cost: 10,
        //             variant: Variant::Argon2id,
        //             version: Version::Version13,
        //             ..Default::default()
        //         };

        //         let charset = "abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
        //         let signing_key = random_string::generate(32, charset);

        //         let salt = signing_key.as_bytes();
        //         let key =
        //             hash_raw(password.as_ref(), salt, &config).expect("failed to hash password");

        //         key.to_vec()
        //     })
        //     .build(),
        // )
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            commands::authentication::sign_in,
            commands::vaults::get_all_vaults,
            commands::vaults::create_vault
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

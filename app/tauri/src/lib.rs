#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri_plugin_sql::{Migration, MigrationKind};

pub fn run() {
    let migrations = vec![Migration {
        version: 1,
        description: "create_vault_entries_table",
        sql: "CREATE TABLE vault_entries (id VARCHAR PRIMARY KEY, title TEXT, description TEXT, keywords TEXT, date_added TEXT, last_modified TEXT);",
        kind: MigrationKind::Up
    },
   Migration {
    version:2,
  description: "create_vault_access_history",
        sql: "CREATE TABLE vault_access_history (id VARCHAR PRIMARY KEY, action TEXT, device TEXT, date TEXT);",
        kind: MigrationKind::Up
    }
    ];

    tauri::Builder::default()
        // .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:vault.db", migrations)
                .build(),
        )
        // .plugin(tauri_plugin_single_instance::init())
        // .plugin(tauri_plugin_stronghold::Builder::new(|pass| todo!()).build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        // .plugin(tauri_plugin_autostart::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

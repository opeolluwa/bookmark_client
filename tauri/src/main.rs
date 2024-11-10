// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use app_lib::config;
use vault_database::migration::Migrator;
use vault_database::migration::MigratorTrait;
pub mod app_state;
pub mod commands;
pub mod config;
pub mod helpers;
fn main() {
    tauri::async_runtime::spawn(exec_db_migration());

    app_lib::run();
}

async fn exec_db_migration() -> anyhow::Result<()> {
    let connection = sea_orm::Database::connect(&config::CONFIG.database_connection_string).await?;
    Migrator::up(&connection, None).await?;
    Ok(())
}

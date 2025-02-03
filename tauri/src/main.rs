// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

<<<<<<< HEAD

use bookmark_database_codegen::migration::Migrator;
use bookmark_database_codegen::migration::MigratorTrait;
pub mod app_state;
pub mod commands;
pub mod config;
pub mod ipc_manager;

=======
>>>>>>> c075b4af20325c71e73098574c5a918bee903c38
fn main() {
    bookmark_lib::run()
}

use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use rusqlite::Connection;
use std::{env::var, path::PathBuf};

use crate::table_names::{APPLICATION_SETTINGS_TABLE, BOOKMARK_COLLECTION_TABLE};

static RESOURCES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/resources");
static DEVELOPMENT_ENVIRONMENT: &'static str = "development";
static NONE_DEVELOPMENT_ENVIRONMENT: &'static str = "production";
static DEVELOPMENT_DATABASE_FILE_PATH: &'static str = "resources/bookmark.dev.sqlite";
static NON_DEVELOPMENT_DATABASE_FILE_PATH: &'static str = "resources/bookmark.sqlite";

lazy_static! {
    static ref DATABASE_PATH: PathBuf = {
        let application_run_environment =
            var("ENV").unwrap_or(NONE_DEVELOPMENT_ENVIRONMENT.to_string());

        let database_file_path = if application_run_environment.as_str() == DEVELOPMENT_ENVIRONMENT
        {
            // RESOURCES_DIR.get_file(DEVELOPMENT_DATABASE_FILE_PATH)
            DEVELOPMENT_DATABASE_FILE_PATH
        } else {
            // RESOURCES_DIR.get_file(NON_DEVELOPMENT_DATABASE_FILE_PATH)
            NON_DEVELOPMENT_DATABASE_FILE_PATH
        };

        PathBuf::from(database_file_path)
    };
}
pub struct SqliteWasm {}

impl SqliteWasm {
    pub fn init() -> rusqlite::Result<Connection> {
        let database_connection = Connection::open(DATABASE_PATH.as_path())?;

        let create_tables_result =
            database_connection.execute_batch(&Self::create_table_statements());

        if create_tables_result.is_err() {
            return Err(create_tables_result.err().unwrap());
        } else {
            return Ok(database_connection);
        }
    }

    pub fn create_table_statements() -> String {
        format!(
            r#"
        BEGIN;
        CREATE TABLE IF NOT EXISTS {BOOKMARK_COLLECTION_TABLE} (
            id TEXT PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            description TEXT,
            content TEXT, 
            date_added TEXT DEFAULT CURRENT_TIMESTAMP,
            last_modified TEXT DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS {APPLICATION_SETTINGS_TABLE} (
            id INTEGER PRIMARY KEY,
            language TEXT DEFAULT "english",
            theme TEXT DEFAULT "light",
            initialized BOOLEAN DEFAULT FALSE
        );
        COMMIT;
        "#
        )
    }
}

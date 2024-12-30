use std::fs;
use std::path::Path;

use rusqlite::Connection;

use crate::{
    entities::settings::Settings,
    table_names::{APPLICATION_SETTINGS_TABLE, BOOKMARK_COLLECTION_TABLE},
};

pub struct SqliteWasm {}

#[allow(dead_code)]
impl SqliteWasm {
    pub fn init(_database_path: &str) -> rusqlite::Result<Connection> {
        // let database_connection = Connection::open(database_path)?;
        let database_connection = Connection::open_in_memory()?;

        let create_tables_result =
            database_connection.execute_batch(&Self::create_table_statements());

        // default settings and other presets
        Self::create_default_settings(&database_connection);

        if create_tables_result.is_err() {
            return Err(create_tables_result.err().unwrap());
        } else {
            return Ok(database_connection);
        }
    }

    // / Create the database file.
    fn create_db_file() {
        let db_path = Self::get_db_path();
        let db_dir = Path::new(&db_path).parent().unwrap();

        // If the parent directory does not exist, create it.
        if !db_dir.exists() {
            fs::create_dir_all(db_dir).unwrap();
        }

        // Create the database file.
        fs::File::create(db_path).unwrap();
    }

    // Check whether the database file exists.
    fn db_file_exists() -> bool {
        let db_path = Self::get_db_path();
        Path::new(&db_path).exists()
    }

    // Get the path where the database file should be located.
    fn get_db_path() -> String {
        // let home_dir = dirs::home_dir().unwrap();
        // home_dir.to_str().unwrap().to_string() + "/.config/bookmark/bookmark.sqlite"
        "/.config/bookmark/bookmark.sqlite".to_string()
    }

    fn create_table_statements() -> String {
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

    fn create_default_settings(conn: &Connection) {
        Settings::new("english", "light").save(conn);
    }
}

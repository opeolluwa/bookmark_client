use crate::table_names::{APPLICATION_SETTINGS_TABLE, BOOKMARK_COLLECTION_TABLE};
use gluesql::{gluesql_memory_storage::MemoryStorage, prelude::Glue};
pub struct SqliteWasm {}

impl SqliteWasm {
    pub async fn init() {
        let storage = MemoryStorage::default();
        let mut gluesql_db_instance = Glue::new(storage);
        for statement in Self::create_table_statements().into_iter() {
            gluesql_db_instance
                .execute(statement)
                .await
                .expect("error initializing storage engine");
        }
    }

    fn create_table_statements() -> Vec<String> {
        let create_bookmark_collection_table = format!(
            r#"
        CREATE TABLE IF NOT EXISTS {table_name} (
            id UUID PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            description TEXT,
            date_added DATE,
            last_modified DATE
        )
        "#,
            table_name = BOOKMARK_COLLECTION_TABLE
        );

        let create_application_settings_table = format!(
            r#"
         CREATE TABLE IF NOT EXISTS {table_name} (
            id UUID PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            description TEXT,
            date_added DATE,
            last_modified DATE
        )
        "#,
            table_name = APPLICATION_SETTINGS_TABLE
        );

        vec![
            create_bookmark_collection_table,
            create_application_settings_table,
        ]
    }
}

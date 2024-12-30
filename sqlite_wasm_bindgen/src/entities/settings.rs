use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

use crate::table_names::APPLICATION_SETTINGS_TABLE;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    id: i32,
    language: String,
    theme: String,
    initialized: bool,
}

impl Settings {
    pub fn new(language: &str, theme: &str) -> Self {
        Self {
            id: 1,
            language: language.to_string(),
            theme: theme.to_string(),
            initialized: false,
        }
    }

    pub fn save(&self, conn: &Connection) {
        conn.execute(
            &format!(r#"
            INSERT INTO {APPLICATION_SETTINGS_TABLE} (id, language, theme, initialized) VALUES (?1, ?2, ?3, ?4)"#),
            &[&self.id.to_string(), &self.language, &self.theme, &self.initialized.to_string()],
        )
        .unwrap();
    }

    pub fn fetch(conn: &Connection) -> Result<Self> {
        let mut statement = conn.prepare(&format!(
            r#"SELECT id, language, theme, initialized FROM {APPLICATION_SETTINGS_TABLE} WHERE id = 1"#
        ))?;

        let app_settings = statement.query_row([], |row| {
            print!("{:#?}", row);
            let initialized: String = row.get(3)?;
            let initialized = initialized.parse::<bool>().unwrap();

            Ok(Settings {
                id: row.get(0)?,
                language: row.get(1)?,
                theme: row.get(2)?,
                initialized,
            })
        })?;

        Ok(app_settings)
    }

    // change the application initialization status
    pub fn change_initialization_status(status: bool, conn: &Connection) {
        conn.execute(
            &format!(
                r#"
            UPDATE {APPLICATION_SETTINGS_TABLE} SET initialized = ?1 WHERE id = 1
            "#,
            ),
            &[&status.to_string()],
        )
        .unwrap();
    }
}

#[cfg(test)]
mod test {

    use super::Settings;

    #[test]
    fn test_settings() {
        let connections =
            rusqlite::Connection::open_in_memory().expect("Failed to open connection");

        connections
            .execute_batch(
                r#"
            BEGIN;
            CREATE TABLE IF NOT EXISTS application_settings (
                id INTEGER PRIMARY KEY,
                language TEXT DEFAULT "english",
                theme TEXT DEFAULT "light",
                initialized BOOLEAN DEFAULT FALSE
            );
            COMMIT;
            "#,
            )
            .expect("Failed to create table");

        let app_settings = Settings::new("english", "light");

        app_settings.save(&connections);

        let fetched_settings = Settings::fetch(&connections).expect("Failed to fetch settings");

        assert_eq!(app_settings.language, fetched_settings.language);
    }
}

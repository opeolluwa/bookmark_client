use include_dir::{include_dir, Dir};
use rusqlite::{Connection, Result};

use crate::models::application_settings::ApplicationSettings;

static RESOURCE_PATH: Dir = include_dir!("$CARGO_MANIFEST_DIR");
static RELEASE_ENV: &'static str = "release";
static TEST_DB_PATH: &'static str = "resource/test.db";
static RELEASE_DB_PATH: &'static str = "resource/prod.temp.db";

pub struct SqliteWasm {}

impl SqliteWasm {
    /// create a new connection and return the connection
    pub fn init() -> Result<()> {
        let application_environment =
            std::env::var("APPLICATION_ENV").unwrap_or(RELEASE_ENV.to_string());
        let db_path = if application_environment == RELEASE_ENV {
            RELEASE_DB_PATH
        } else {
            TEST_DB_PATH
        };

        let connection = Connection::open(db_path)?;
        let _ = ApplicationSettings::create_table(connection);
        Ok(())
    }
}

pub trait CreateTable {
    fn create_table(connection: Connection) -> Result<()>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_for_embedded_test_database() {
        assert!(RESOURCE_PATH.get_file("resources/test.db").is_some());
    }

    #[test]
    fn check_for_embedded_prod_database() {
        assert!(RESOURCE_PATH.get_file("resources/prod.temp.db").is_some());
    }
}

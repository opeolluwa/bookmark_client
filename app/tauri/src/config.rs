use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    /// database url for the embedded Sqlite database
    pub database_connection_string: String,
    /// jwt key
    pub jwt_signing_key: String,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "database_connection_string:{}",
            self.database_connection_string
        )
    }
}

impl Config {
    pub fn parse() -> Self {
        let database_connection_string = {
            let db_path = "resources".to_string();
            let _ = std::fs::create_dir_all(&db_path);
            format!("sqlite://{db_path}/mobile.vault.db?mode=rwc")
        };

        let jwt_signing_key = "nr2CRwaADKauevLmN9KdRG482k1gWkl7cb51".to_string(); //TODO: use randog key generation
        Self {
            database_connection_string,
            jwt_signing_key,
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::parse();
}

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    /// database url for the embedded Sqlite database
    pub database_connection_string: String,
    /// jwt key
    pub jwt_signing_key: String,
    pub api_base_url: String,
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
            format!("sqlite://{db_path}/bookmark.backup.db?mode=rwc")
        };

        let jwt_signing_key = "nr2CRwaADKauevLmN9KdRG482k1gWkl7cb51".to_string(); //TODO: use random key generation
        Self {
            database_connection_string,
            jwt_signing_key,
            api_base_url: "http://localhost:4576".to_string(),
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::parse();
}

pub fn request_endpoint(endpoint: &str) -> String {
    format!("{base_url}/{endpoint}", base_url = CONFIG.api_base_url)
}

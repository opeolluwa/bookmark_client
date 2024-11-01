use std::time::Duration;

use sea_orm::{ConnectOptions, Database};

use crate::config;

pub struct DatabaseConnection {}

impl DatabaseConnection {
    pub async fn new() -> sea_orm::DatabaseConnection {
        let mut opt = ConnectOptions::new(&config::CONFIG.database_connection_string);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(20))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .set_schema_search_path("");

        Database::connect(opt)
            .await
            .expect("Couldn't connect to database")
    }
}

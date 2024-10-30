use crate::config;
use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use std::time::Duration;

#[derive(Debug)]
pub struct AppState {
    // database
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn from(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

pub async fn db_connection() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(&config::CONFIG.database_connection_string);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        // .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("mobile.vault");

    Database::connect(opt).await.unwrap()
}

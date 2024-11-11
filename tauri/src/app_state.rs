use crate::config;
use reddb::serializer::Json;
use reddb::FileStorage;
use reddb::JsonDb;
use reddb::RedDb;
use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;

#[derive(Debug)]
pub struct AppState {
    pub grpc_server: String,
    pub token_store: RedDb<JsonDb, FileStorage<Json>>,
}

// impl AppState {
//     pub fn new() -> Self {
//         Self {
//             ..Default::default()
//         }
//     }
//     pub fn with_grpc_server(&self, server_uri: &str) -> Self {
//         Self {
//             grpc_server: server_uri.to_string(),
//             token_store: self.token_store,
//         }
//     }
//     pub fn with_token_store(&self, store: RedDb<JsonDb, FileStorage<Json>>) -> Self {
//         Self {
//             grpc_server: self.grpc_server,
//             token_store: store,
//         }
//     }
// }

pub async fn db_connection() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(&config::CONFIG.database_connection_string);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .set_schema_search_path("mobile.vault");

    Database::connect(opt).await.unwrap()
}

#[derive(Clone, Debug, Serialize, PartialEq, Deserialize)]
pub struct TokenStore {
    token: String,
}

impl TokenStore {
    pub fn from(token: &str) -> Self {
        Self {
            token: token.trim().to_string(),
        }
    }
}

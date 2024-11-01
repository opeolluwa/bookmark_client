use std::{
    net::{Ipv4Addr, SocketAddrV4},
    time::Duration,
};

use crate::config::CONFIG;
use anyhow::Ok;
use sea_orm::{ConnectOptions, Database};
use tokio::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub struct Server;

impl Server {
    pub async fn run() -> anyhow::Result<()> {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                    "mobile_vault_server_logging=debug,tower_http=info,axum::rejection=trace".into()
                }),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();

        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, CONFIG.port);
        let mut database_connection_options =
            ConnectOptions::new(&CONFIG.database_connection_string);

        database_connection_options
            .max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let db = Database::connect(database_connection_options).await?;
        let app_state = super::app_state::AppState::from(&db);
        let app = super::routes::router().with_state(app_state);
        let listener = TcpListener::bind(addr).await?;

        tracing::debug!("server running on http://{addr}");
        axum::serve(listener, app).await?;

        Ok(())
    }
}

use std::net::{Ipv4Addr, SocketAddrV4};

use crate::config::CONFIG;
use anyhow::Ok;
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

        let app = super::routes::router();
        let listener = TcpListener::bind(addr).await?;

        tracing::debug!("server running on http://{addr}");
        axum::serve(listener, app).await?;

        Ok(())
    }
}

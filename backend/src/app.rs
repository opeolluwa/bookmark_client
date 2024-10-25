use std::net::{Ipv4Addr, SocketAddrV4};

use anyhow::Ok;
use sea_orm::{ConnectOptions, Database};
use tokio::net::TcpListener;

use crate::{config::CONFIG, state::AppState};

pub struct Server;

impl Server {
    pub async fn run() -> anyhow::Result<()> {
        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, CONFIG.port);
        let mut database_connection_options =
            ConnectOptions::new(&CONFIG.database_connection_string);

        database_connection_options.sqlx_logging(true);

        let db = Database::connect(database_connection_options).await?;
        let app_state = AppState::from(&db);
        let app = crate::routes::router().with_state(app_state);
        let listener = TcpListener::bind(addr).await?;

        axum::serve(listener, app).await?;

        Ok(())
    }
}

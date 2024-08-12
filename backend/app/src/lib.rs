use std::time::Duration;

use axum::{response::Html, routing::get, Router};
use migration::{Migrator, MigratorTrait};
use sea_orm::ConnectOptions;

pub struct Vault;

impl Vault {
    pub async fn run() -> anyhow::Result<()> {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .init();

        // build our application with a routeØ
        let app = Router::new().route("/", get(handler));
        let database_url = std::env::var("DATABASE_URL").unwrap_or("postgres://drizzles:thunderstorm@localhost/vault".to_string());
        let mut opt = ConnectOptions::new(&database_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true); // Setting default PostgreSQL schema

        let connection = sea_orm::Database::connect(&database_url).await?;
        Migrator::up(&connection, None).await?;
        // run it
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap();
        println!("listening on http://{}", listener.local_addr().unwrap());
        axum::serve(listener, app).await?;

        Ok(())
    }
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

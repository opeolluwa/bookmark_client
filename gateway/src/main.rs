pub mod api_response;
pub mod app_state;
pub mod config;
pub mod error;
pub mod handlers;
pub mod helpers;
pub mod proto;
pub mod routes;
pub mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    self::server::Server::run().await
}

pub mod config;
// pub mod error;
pub mod grpc_service;
pub mod proto;

use self::config::CONFIG;
use grpc_service::activity_log::ActivityLogImplementation;
use grpc_service::authentication::AuthenticationImplementation;
use grpc_service::health_check::HealthCheckImplementation;
use migration::{Migrator, MigratorTrait};

use proto::activity_log::activity_log_server::ActivityLogServer;
use proto::authentication::authentication_server::AuthenticationServer;
use proto::health_check::health_check_server::HealthCheckServer;
use tonic::transport::Server;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "mobile_vault_server_logging=debug,tower_http=info,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let connection = sea_orm::Database::connect(&CONFIG.database_connection_string).await?;
    Migrator::up(&connection, None).await?;

    // let addr = "[::1]:50051".parse().unwrap();
    let addr = "127.0.0.1:50051".parse().unwrap();

    let activity_log = ActivityLogServer::new(ActivityLogImplementation::default());
    let authentication = AuthenticationServer::new(AuthenticationImplementation::default());
    let health_check = HealthCheckServer::new(HealthCheckImplementation::default());

    Server::builder()
        .add_service(authentication)
        .add_service(activity_log)
        .add_service(health_check)
        .serve(addr)
        .await?;

    Ok(())
}

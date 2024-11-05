pub mod config;
pub mod database_connection;
pub mod grpc_service;
pub mod interceptors;
pub mod jwt;
pub mod proto;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use self::config::CONFIG;
use grpc_service::activity_log::ActivityLogImplementation;
use grpc_service::authentication::AuthenticationImplementation;
use grpc_service::health_check::HealthCheckImplementation;
use grpc_service::user_profile::UserProfileImplementation;
use interceptors::authentication::check_and_validate_jwt;
use migration::{Migrator, MigratorTrait};

use proto::activity_log::activity_log_server::ActivityLogServer;
use proto::authentication::authentication_server::AuthenticationServer;
use proto::health_check::health_check_server::HealthCheckServer;
use proto::user_profile::user_profile_server::UserProfileServer;
use tonic::transport::Server;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
    //             "mobile_vault_server_logging=debug,tower_http=info,axum::rejection=trace".into()
    //         }),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let connection = sea_orm::Database::connect(&CONFIG.database_connection_string).await?;
    Migrator::up(&connection, None).await?;

    let addr = SocketAddr::new(IpAddr::from(Ipv4Addr::LOCALHOST), 50051);

    let activity_log = ActivityLogServer::new(ActivityLogImplementation::default());
    let authentication = AuthenticationServer::new(AuthenticationImplementation::default());
    let health_check = HealthCheckServer::new(HealthCheckImplementation::default());

    let user_profile = UserProfileImplementation::default();
    let user_profile = UserProfileServer::with_interceptor(user_profile, check_and_validate_jwt);

    Server::builder()
        .add_service(authentication)
        .add_service(activity_log)
        .add_service(health_check)
        .add_service(user_profile)
        .serve(addr)
        .await?;

    Ok(())
}

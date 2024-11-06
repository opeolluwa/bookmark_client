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
use grpc_service::vault::VaultManagerImplementation;
use grpc_service::vault_entries::VaultEntriesManagerImplementation;
use interceptors::authentication::check_and_validate_jwt;
use migration::{Migrator, MigratorTrait};

use proto::activity_log::activity_log_server::ActivityLogServer;
use proto::authentication::authentication_server::AuthenticationServer;
use proto::health_check::health_check_server::HealthCheckServer;
use proto::user_profile::user_profile_server::UserProfileServer;
use proto::vault::vault_manager_server::VaultManagerServer;
use proto::vault_entries::vault_entries_manager_server::VaultEntriesManagerServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let connection = sea_orm::Database::connect(&CONFIG.database_connection_string).await?;
    Migrator::up(&connection, None).await?;

    let addr = SocketAddr::new(IpAddr::from(Ipv4Addr::UNSPECIFIED), 50051);

    let activity_log_service = ActivityLogServer::new(ActivityLogImplementation::default());
    let authentication_service = AuthenticationServer::new(AuthenticationImplementation::default());
    let health_check_service = HealthCheckServer::new(HealthCheckImplementation::default());

    let user_profile = UserProfileImplementation::default();
    let user_profile_service =
        UserProfileServer::with_interceptor(user_profile, check_and_validate_jwt);

    let vault_manager = VaultManagerImplementation::default();
    let vault_manager_service =
        VaultManagerServer::with_interceptor(vault_manager, check_and_validate_jwt);

    let vault_entries_manager = VaultEntriesManagerImplementation::default();
    let vault_entries_manager_service =
        VaultEntriesManagerServer::with_interceptor(vault_entries_manager, check_and_validate_jwt);

    Server::builder()
        .add_service(authentication_service)
        .add_service(activity_log_service)
        .add_service(health_check_service)
        .add_service(user_profile_service)
        .add_service(vault_manager_service)
        .add_service(vault_entries_manager_service)
        .serve(addr)
        .await?;

    Ok(())
}

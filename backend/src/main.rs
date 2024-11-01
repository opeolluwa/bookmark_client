pub mod config;
pub mod error;
pub mod grpc_service;
pub mod http_service;
pub mod multiplex;
pub mod proto;

use std::time::Duration;

use self::config::CONFIG;
use http_service::app_state::AppState;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};

use crate::multiplex::GrpcMultiplexLayer;
use axum::routing::get;
use axum::Router;
use tonic::transport::Server;
use tonic_reflection::pb::v1::FILE_DESCRIPTOR_SET;
use tower::util::ServiceExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

// use self::multiplex::MultiplexService;

// use grpc::authentication::{authentication_server::Authentication authentication_server::AuthenticationServer};

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let connection = sea_orm::Database::connect(&CONFIG.database_connection_string).await?;

//     Migrator::up(&connection, None).await?;

//     pkg::app::Server::run().await
// }

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
    let mut database_connection_options = ConnectOptions::new(&CONFIG.database_connection_string);

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
    let app_state = http_service::app_state::AppState::from(&db);
    let _http_layer: Router<AppState> = http_service::routes::router().with_state(app_state);
    // let listener = TcpListener::bind(addr).await?;

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;

    let web = Router::new()
        .route("/test", get(|| async { "Hello, World!" }))
        .into_service()
        .map_response(|r| r.map(tonic::body::boxed));

    let grpc = Server::builder()
        .layer(GrpcMultiplexLayer::new(web))
        .add_service(reflection_service);
    // .add_service(grpc_service::authentication::AuthenticationImplementation::default());

    let addr = std::net::SocketAddrV4::new(std::net::Ipv4Addr::UNSPECIFIED, CONFIG.port);

    grpc.serve(std::net::SocketAddr::V4(addr)).await?;

    Ok(())
}

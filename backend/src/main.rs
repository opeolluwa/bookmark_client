pub mod grpc;
pub mod multiplex;
pub mod proto;

use migration::{Migrator, MigratorTrait};
use pkg::config::CONFIG;

use crate::multiplex::GrpcMultiplexLayer;
use axum::routing::get;
use axum::Router;
use tonic::transport::Server;
use tower::util::ServiceExt;
 use tonic_reflection::pb::v1::FILE_DESCRIPTOR_SET;
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
    let connection = sea_orm::Database::connect(&CONFIG.database_connection_string).await?;

    Migrator::up(&connection, None).await?;

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()?;

    let web = Router::new()
        .route("/test", get(|| async { "Hello, World!" }))
        .into_service()
        .map_response(|r| r.map(tonic::body::boxed));

    let grpc = Server::builder()
        .layer(GrpcMultiplexLayer::new(web))
        .add_service(reflection_service);

    let addr = "[::1]:50051".parse().unwrap();

    grpc.serve(addr).await?;

    Ok(())
}

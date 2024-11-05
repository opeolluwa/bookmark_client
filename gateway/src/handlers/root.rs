use axum::{extract::State, Json};
use tonic::Request;

use crate::{
    app_state::AppState,
    proto::health_check::{health_check_client::HealthCheckClient, HealthCheckRequest},
};

pub async fn base() -> &'static str {
    "The application is running on https://localhost:3000"
}

// health check
pub async fn health_check_handler(State(app_state): State<AppState>) -> &'static str {
    let channel = app_state.grpc_channel.connect().await.unwrap();
    let mut health_check_grpc_client = HealthCheckClient::new(channel.clone());

    let request = Request::new(HealthCheckRequest {});

    let response = health_check_grpc_client
        .check_service_health(request)
        .await
        .unwrap();

    let (_, body, _) = response.into_parts();
    println!("{:#?}", body);

    // Json(body)
    "Service is healthy"
}

use tonic::transport::Endpoint;

#[derive(Debug, Clone)]
pub struct AppState {
    pub grpc_channel: Endpoint,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            grpc_channel: Endpoint::from_static("http://127.0.0.1:50051"),
        }
    }
}

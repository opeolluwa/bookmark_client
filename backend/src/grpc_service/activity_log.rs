use crate::proto::activity_log::{
    activity_log_server::ActivityLog, LogActivityRequest, LogActivityResponse,
};

#[derive(Default, Clone)]
pub struct ActivityLogImplementation {}

#[tonic::async_trait]
impl ActivityLog for ActivityLogImplementation {
    async fn log_activity(
        &self,
        _request: tonic::Request<LogActivityRequest>,
    ) -> std::result::Result<tonic::Response<LogActivityResponse>, tonic::Status> {
        unimplemented!()
    }
}

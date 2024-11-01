use crate::proto::authentication::authentication_server::Authentication;
use crate::proto::authentication::LoginRequest;
use crate::proto::authentication::LoginResponse;
use crate::proto::authentication::ProfileRequest;
use crate::proto::authentication::ProfileResponse;
use crate::proto::authentication::ProfileUpdateRequest;
use crate::proto::authentication::ProfileUpdateResponse;
// use crate::proto::authentication::ProfileRequest;
#[derive(Default)]
pub struct AuthenticationImplementation {}

#[tonic::async_trait]
impl Authentication for AuthenticationImplementation {
    async fn login(
        &self,
        request: tonic::Request<LoginRequest>,
    ) -> std::result::Result<tonic::Response<LoginResponse>, tonic::Status> {
        unimplemented!()
    }
    async fn get_profile(
        &self,
        request: tonic::Request<ProfileRequest>,
    ) -> std::result::Result<tonic::Response<ProfileResponse>, tonic::Status> {
        unimplemented!()
    }
    async fn update_profile(
        &self,
        request: tonic::Request<ProfileUpdateRequest>,
    ) -> std::result::Result<tonic::Response<ProfileUpdateResponse>, tonic::Status> {
        unimplemented!()
    }
}

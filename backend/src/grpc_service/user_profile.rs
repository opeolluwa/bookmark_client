use tonic::async_trait;

use crate::{
    database_connection::DatabaseConnection,
    proto::user_profile::{
        user_profile_server::UserProfile, ProfileRequest, ProfileResponse, ProfileUpdateRequest,
        ProfileUpdateResponse,
    },
};

#[derive(Default)]
pub struct UserProfileImplementation {}

#[async_trait]

impl UserProfile for UserProfileImplementation {
    async fn get_profile(
        &self,
        request: tonic::Request<ProfileRequest>,
    ) -> std::result::Result<tonic::Response<ProfileResponse>, tonic::Status> {
        let (metadata, _, payload) = request.into_parts();
        // let metadata = request.metadata();
        let db_connection = &DatabaseConnection::new().await;

        println!("{:#?}", metadata);
        todo!()
    }
    async fn update_profile(
        &self,
        request: tonic::Request<ProfileUpdateRequest>,
    ) -> std::result::Result<tonic::Response<ProfileUpdateResponse>, tonic::Status> {
        todo!()
    }
}

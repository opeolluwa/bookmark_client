use crate::{
    database_connection::DatabaseConnection,
    proto::user_profile::{
        user_profile_server::UserProfile, ProfileRequest, ProfileResponse, ProfileUpdateRequest,
        ProfileUpdateResponse,
    },
};
use entities::prelude::*;
use entities::user_information::{self};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use tonic::async_trait;

#[derive(Default)]
pub struct UserProfileImplementation {}

#[async_trait]

impl UserProfile for UserProfileImplementation {
    async fn get_profile(
        &self,
        request: tonic::Request<ProfileRequest>,
    ) -> std::result::Result<tonic::Response<ProfileResponse>, tonic::Status> {
        let (metadata, _, _) = request.into_parts();
        let db_connection = &DatabaseConnection::new().await;

        let Some(user_email) = metadata.get("user_email") else {
            return Err(tonic::Status::unauthenticated(
                "Missing or badly formatted authorization header",
            ));
        };

        let user_email = user_email.to_str().map_err(|_| {
            tonic::Status::not_found("Missing or badly formatted authorization header")
        })?;

        let Some(user_data) = UserInformation::find()
            .filter(user_information::Column::Email.eq(user_email))
            .one(db_connection)
            .await
            .map_err(|_| {
                tonic::Status::not_found("A user with the provided email does not exist")
            })?
        else {
            return Err(tonic::Status::not_found(
                "A user with the provided email does not exist",
            ));
        };

        let message = ProfileResponse {
            email: user_data.email,
            last_name: user_data.last_name,
            first_name: user_data.first_name,
            id: user_data.id.to_string(),
        };

        Ok(tonic::Response::new(message))
    }
    async fn update_profile(
        &self,
        request: tonic::Request<ProfileUpdateRequest>,
    ) -> std::result::Result<tonic::Response<ProfileUpdateResponse>, tonic::Status> {
        let (metadata, _, payload) = request.into_parts();
        let db_connection = &DatabaseConnection::new().await;

        let Some(user_email) = metadata.get("user_email") else {
            return Err(tonic::Status::unauthenticated(
                "Missing or badly formatted authorization header",
            ));
        };

        let user_email = user_email.to_str().map_err(|_| {
            tonic::Status::not_found("Missing or badly formatted authorization header")
        })?;

        println!("{}", user_email);

        let user_data = UserInformation::find()
            .filter(user_information::Column::Email.eq(user_email))
            .one(db_connection)
            .await
            .map_err(|err| {
                log::error!("Error fetching user_information{:#?}", err);
                tonic::Status::not_found("A user with the provided email does not exist")
            })?
            .unwrap();
        // else {
        //     println!("hehehe");
        //     return Err(tonic::Status::not_found(
        //         "A user with the provideddddddddd email does not exist",
        //     ));
        // };

        // update the fields in the update was sent or use the prev val
        let first_name = &user_data.first_name;
        let last_name = &user_data.last_name;
        let email = &user_data.email;

        let mut user_data: user_information::ActiveModel = user_data.to_owned().into();
        user_data.first_name = Set(payload.first_name.unwrap_or(first_name.to_string()));
        user_data.last_name = Set(payload.last_name.unwrap_or(last_name.to_string()));
        user_data.email = Set(payload.email.unwrap_or(email.to_string()));

        let updated_user_data: user_information::Model = user_data
            .update(db_connection)
            .await
            .map_err(|_| tonic::Status::internal("couldn't process request at this time "))?;

        let message = ProfileUpdateResponse {
            email: updated_user_data.email,
            last_name: updated_user_data.last_name,
            first_name: updated_user_data.first_name,
            id: updated_user_data.id.to_string(),
        };

        Ok(tonic::Response::new(message))
    }
}

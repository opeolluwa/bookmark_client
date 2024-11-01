use crate::database_connection::DatabaseConnection;
use crate::jwt::JwtClaims;
use crate::proto::authentication::authentication_server::Authentication;
use crate::proto::authentication::LoginRequest;
use crate::proto::authentication::LoginResponse;
use crate::proto::authentication::ProfileRequest;
use crate::proto::authentication::ProfileResponse;
use crate::proto::authentication::ProfileUpdateRequest;
use crate::proto::authentication::ProfileUpdateResponse;
use crate::proto::authentication::SignUpRequest;
use crate::proto::authentication::SignUpResponse;
use crate::proto::authentication::Status as RequestStatus;

use bcrypt::{verify, DEFAULT_COST};
use entities::prelude::*;
use entities::user_information::{self};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;
use tonic::Response;
use tonic::Status;
use uuid::Uuid;

#[derive(Default)]
pub struct AuthenticationImplementation {}

#[tonic::async_trait]
impl Authentication for AuthenticationImplementation {
    async fn sign_up(
        &self,
        request: tonic::Request<SignUpRequest>,
    ) -> std::result::Result<tonic::Response<SignUpResponse>, tonic::Status> {
        let payload = request.into_inner();
        let db_connection = &DatabaseConnection::new().await;

        if UserInformation::find()
            .filter(user_information::Column::Email.eq(&payload.email))
            .one(db_connection)
            .await
            .ok()
            .unwrap()
            .is_some()
        {
            return Err(tonic::Status::already_exists(
                "A user with the provided email already exist",
            ));
        }

        let password = bcrypt::hash(payload.password, DEFAULT_COST)
            .map_err(|_| {
                return Err::<String, Status>(tonic::Status::unknown(
                    "the server couldn't process the request at this time",
                ));
            })
            .unwrap();
        let new_user = user_information::ActiveModel {
            id: Set(Uuid::new_v4()),
            password: Set(password),
            first_name: Set(payload.first_name.trim().to_string().to_lowercase()),
            last_name: Set(payload.last_name.trim().to_string().to_lowercase()),
            email: Set(payload.email.trim().to_string().to_lowercase()),
            ..Default::default()
        };

        if let Err(res) = user_information::Entity::insert(new_user)
            .exec(db_connection)
            .await
        {
            return Err(tonic::Status::unknown(res.to_string()));
        }

        let message = SignUpResponse {
            message: "Account Successfully Created".into(),
            status: RequestStatus::Ok.into(),
            error: None,
        };
        Ok(Response::new(message))
    }
    async fn login(
        &self,
        request: tonic::Request<LoginRequest>,
    ) -> std::result::Result<tonic::Response<LoginResponse>, tonic::Status> {
        let payload = request.into_inner();
        let db_connection = &DatabaseConnection::new().await;

        let user_data = UserInformation::find()
            .filter(user_information::Column::Email.eq(&payload.email))
            .one(db_connection)
            .await
            .unwrap()
            .ok_or_else(|| todo!())
            .unwrap();

        let Some(is_correct_password) = verify(payload.password, &user_data.password).ok() else {
            todo!()
        };

        if !is_correct_password {
            todo!()
        }

        // sign the token
        let Ok(jwt_token) =
            JwtClaims::new(user_data.email.clone(), user_data.id.clone().to_string()).gen_token()
        else {
            todo!()
        };

        let message: LoginResponse = LoginResponse {
            token: jwt_token,
            message: "User successfully logged in".into(),
            error: None,
        };

        Ok(Response::new(message))
    }
    async fn get_profile(
        &self,
        _request: tonic::Request<ProfileRequest>,
    ) -> std::result::Result<tonic::Response<ProfileResponse>, tonic::Status> {
        unimplemented!()
    }
    async fn update_profile(
        &self,
        _request: tonic::Request<ProfileUpdateRequest>,
    ) -> std::result::Result<tonic::Response<ProfileUpdateResponse>, tonic::Status> {
        unimplemented!()
    }
}

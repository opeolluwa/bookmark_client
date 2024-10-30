use crate::{
    error::AppError,
    jwt::JwtClaims,
    shared::{ApiResponse, IntoApiResponse, ResponseBody},
    state::AppState,
};
use axum::{extract::State, http::StatusCode, Json};
use bcrypt::{verify, DEFAULT_COST};
use entities::prelude::*;
use entities::user_information::{self};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;
use validator::Validate;

pub async fn register_user(
    State(app_state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> ApiResponse<ResponseBody<Value>> {
    if let Err(error_message) = payload.validate() {
        return Err(AppError::BadCredentialsError {
            message: Some(error_message.to_string()),
        });
    }

    if UserInformation::find()
        .filter(user_information::Column::Email.eq(&payload.email))
        .one(&app_state.db)
        .await
        .ok()
        .unwrap()
        .is_some()
    {
        return Err(AppError::ConflictError {
            message: Some("A user with the provided email already exist!".to_string()),
        });
    }

    let password = bcrypt::hash(payload.password, DEFAULT_COST)
        .map_err(|_| AppError::ServerError { message: None })?;
    let new_user = user_information::ActiveModel {
        id: Set(Uuid::new_v4()),
        password: Set(password),
        first_name: Set(payload.first_name.trim().to_string().to_lowercase()),
        last_name: Set(payload.last_name.trim().to_string().to_lowercase()),
        email: Set(payload.email.trim().to_string().to_lowercase()),
        ..Default::default()
    };

    if let Err(res) = user_information::Entity::insert(new_user)
        .exec(&app_state.db)
        .await
    {
        return Err(AppError::DatabaseError {
            message: Some(res.to_string()),
        });
    }

    Ok(ApiResponse::from_parts(
        json!({"message":"Account created successfully"}),
        Some(StatusCode::CREATED),
    ))
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> ApiResponse<ResponseBody<Value>> {

    let user_data = UserInformation::find()
        .filter(user_information::Column::Email.eq(&payload.email))
        .one(&app_state.db)
        .await
        .unwrap()
        .ok_or_else(|| AppError::NotFoundError {
            message: Some("Invalid email or password".to_string()),
        })?;

    let Some(is_correct_password) = verify(payload.password, &user_data.password).ok() else {
        return Err(AppError::WrongCredentialsError { message: None });
    };

    if !is_correct_password {
        return Err(AppError::WrongCredentialsError { message: None });
    }

    // sign the token
    let Ok(jwt_token) =
        JwtClaims::new(user_data.email.clone(), user_data.id.clone().to_string()).gen_token()
    else {
        return Err(AppError::ServerError { message: None });
    };

    let response_body = json!({"jwt_token": jwt_token});

    Ok(ApiResponse::from_parts(response_body, None))
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

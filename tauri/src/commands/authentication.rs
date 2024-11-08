use crate::app_state::db_connection;
use crate::helpers::ipc_manager::IpcResponseError;
use crate::helpers::ipc_manager::{IpcResponse, IpcResponseStatus};
use crate::helpers::jwt::JwtClaims;
use bcrypt::verify;
use bcrypt::{hash, DEFAULT_COST};
use entities::prelude::*;
use entities::user_information::{self};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::JsonValue;
use sea_orm::QueryFilter;
use sea_orm::Set;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use ts_rs::TS;
use uuid::Uuid;
use validator::Validate;
use vault_database::entities;

#[tauri::command]
pub async fn sign_up(user: SignUpData) -> IpcResponse<()> {
    let validation_result = user.validate();

    match validation_result {
        Ok(_) => {
            let Some(password_hash) = hash(&user.password, DEFAULT_COST).ok() else {
                return IpcResponse::error("error creating account");
            };

            let new_user = user_information::ActiveModel {
                id: Set(Uuid::new_v4()),
                password: Set(password_hash),
                first_name: Set(user.first_name.to_string()),
                last_name: Set(user.last_name.to_string()),
                email: Set(user.email.to_string()),
                ..Default::default()
            };
            let res = user_information::Entity::insert(new_user)
                .exec(&db_connection().await)
                .await;

            if res.is_err() {
                return IpcResponse::error("database error");
            }
            IpcResponse::success("user added successfully", ())
        }
        Err(error_message) => IpcResponse::error(&error_message.to_string()),
    }
}

#[tauri::command]
pub async fn sign_in(payload: LoginData) -> Result<IpcResponse<JsonValue>, IpcResponse<()>> {
    match payload.validate() {
        Ok(_) => {
            let Ok(Some(user_data)) = UserInformation::find()
                .filter(user_information::Column::Email.eq(&payload.email))
                .one(&db_connection().await)
                .await
            else {
                return Err(IpcResponse::error(
                    "user with the provided email does not exist",
                ));
            };

            let Some(is_correct_password) = verify(payload.password, &user_data.password).ok()
            else {
                return Err(IpcResponse::error("incorrect username or password"));
            };

            if !is_correct_password {
                return Err(IpcResponse::error("incorrect username or password"));
            }

            // sign the token
            let Ok(jwt_token) =
                JwtClaims::new(user_data.email.clone(), user_data.id.clone().to_string())
                    .gen_token()
            else {
                return Err(IpcResponse::error("error processing response"));
            };

            let response_body = json!({"jwt_token": jwt_token});

            Ok(IpcResponse::new(
                response_body,
                "user successfully logged in",
                IpcResponseStatus::Error,
            ))
        }
        Err(error_message) => Err(IpcResponse::error(&error_message.to_string())),
    }
}

#[tauri::command]
pub async fn authenticate() {}

#[derive(Debug, Serialize, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(TS)]
#[ts(export)]
pub struct SignUpData {
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(TS)]
#[ts(export)]
pub struct LoginData {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(TS)]
#[ts(export)]
pub struct PasswordResetRequestData {
    #[validate(email)]
    pub email: String,
}

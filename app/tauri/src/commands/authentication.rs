use crate::api::{IpcResponse, IpcResponseStatus};
use crate::app_state::db_connection;
use bcrypt::{hash, DEFAULT_COST};
use entity::user_information;
use sea_orm::EntityTrait;
use sea_orm::Set;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;
use uuid::Uuid;
use validator::Validate;

#[tauri::command]
pub async fn sign_up(user: SignUpData) -> IpcResponse<()> {
    let validation_result = user.validate();

    match validation_result {
        Ok(_) => {
            let Some(password_hash) = hash(&user.password, DEFAULT_COST).ok() else {
                return IpcResponse::new((), "error creating account", IpcResponseStatus::Error);
            };
            let new_user = user_information::ActiveModel {
                id: Set(Uuid::new_v4()),
                password: Set(password_hash),
                first_name: Set(user.first_name.to_string()),
                last_name: Set(user.last_name.to_string()),
                email: Set(user.email.to_string()),
            };
            let res = user_information::Entity::insert(new_user)
                .exec(&db_connection().await)
                .await;

            if res.is_err() {
                return IpcResponse::new((), "error", IpcResponseStatus::Error);
            }
            IpcResponse::new((), "user added successfully", IpcResponseStatus::Success)
        }
        Err(error_message) => {
            IpcResponse::new((), &error_message.to_string(), IpcResponseStatus::Error)
        }
    }
}

#[tauri::command]
pub async fn sign_in() {}

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

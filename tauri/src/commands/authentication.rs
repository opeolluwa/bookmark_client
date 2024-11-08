use crate::helpers::ipc_manager::CommandResponse;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use ts_rs::TS;
use validator::Validate;
use vault_grpc::client_stub::authentication::LoginResponse;

#[tauri::command]
pub fn sign_in(payload: LoginData) -> CommandResponse<Value> {
    println!("I was invoked from JavaScript! {:#?}", payload);


   let body =  json!({});
    CommandResponse::new(body).set_message("message")
}

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

#[derive(Debug, Serialize, Validate, Deserialize, Default, Clone)]
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
struct PasswordResetRequestData {
    #[validate(email)]
    pub email: String,
}

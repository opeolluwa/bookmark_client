use crate::helpers::ipc_manager::CommandResponse;
use crate::helpers::ipc_manager::CommandResponseStatus;
// use crate::GRPC_SERVER_ENDPOINT;/`         /
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use tonic::Request;
use ts_rs::TS;
use validator::Validate;

use vault_grpc::client_stub::authentication::authentication_client::AuthenticationClient;
use vault_grpc::client_stub::authentication::LoginRequest;
#[tauri::command]
pub async fn sign_in(payload: LoginData) -> Result<CommandResponse<Value>, CommandResponse<()>> {
    let mut client = AuthenticationClient::connect("http://127.0.0.1:50051")
        .await
        .unwrap();

    let LoginData { email, password } = payload;
    let request = Request::new(LoginRequest { email, password });

    let response = client.login(request).await.unwrap().into_inner();

    Ok(CommandResponse::new(json!({
        "token": response.token
    }))
    .set_message(&response.message)
    .set_status(CommandResponseStatus::Success))
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

use crate::helpers::ipc_manager::CommandResponse;
use crate::helpers::ipc_manager::CommandResponseStatus;
use crate::helpers::ipc_manager::CommandResult;
use serde::Deserialize;
use serde::Serialize;
use tonic::Request;
use ts_rs::TS;
use validator::Validate;

use vault_grpc::client_stub::authentication::authentication_client::AuthenticationClient;
use vault_grpc::client_stub::authentication::LoginRequest;
use vault_grpc::client_stub::authentication::LoginResponse;

#[tauri::command]
pub async fn sign_in(payload: LoginRequest) -> CommandResult<LoginResponse> {
    let Some(mut client) = AuthenticationClient::connect("http://127.0.0.1:50051")
        .await
        .ok()
    else {
        return Err(CommandResponse::new(())
            .set_message("Internal server error")
            .set_status(CommandResponseStatus::Error));
    };

    let request = Request::new(payload);
    let response = client
        .login(request)
        .await
        .map_err(|err| {
            CommandResponse::new(())
                .set_message(err.message())
                .set_status(CommandResponseStatus::Error)
        })?
        .into_inner();

    Ok(CommandResponse::new(response.clone())
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

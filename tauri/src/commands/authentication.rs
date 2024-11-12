use crate::helpers::ipc_manager::CommandResponse;
use crate::helpers::ipc_manager::CommandResponseStatus;
use crate::helpers::ipc_manager::CommandResult;
use serde_json::json;
use tauri::Runtime;
use tauri_plugin_store::StoreExt;
use tonic::Request;

use vault_grpc::client_stub::authentication::authentication_client::AuthenticationClient;
use vault_grpc::client_stub::authentication::LoginRequest;

#[tauri::command]
pub async fn sign_in<R: Runtime>(
    app: tauri::AppHandle<R>,
    payload: LoginRequest,
) -> CommandResult<()> {
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

    let store = app.store("store.json").map_err(|_| {
        CommandResponse::new(())
            .set_message("error getting store")
            .set_status(CommandResponseStatus::Error)
    })?;
    store.set("access token", json!({"value":response.token}));
    Ok(CommandResponse::new(()))
}


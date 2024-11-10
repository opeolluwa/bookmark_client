use crate::helpers::ipc_manager::{CommandResponse, CommandResponseStatus, CommandResult};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use tonic::Request;
use ts_rs::TS;
use vault_grpc::client_stub::vault::{vault_manager_client::VaultManagerClient, NewVaultRequest};

#[tauri::command]
async fn create_vault(payload: CreateVaultRequest) -> CommandResult<Value> {
    let Some(mut client) = VaultManagerClient::connect("http://127.0.0.1:50051")
        .await
        .ok()
    else {
        return Err(CommandResponse::new(())
            .set_message("Internal server error")
            .set_status(CommandResponseStatus::Error));
    };

    let CreateVaultRequest { name, description } = payload;
    let request = Request::new(NewVaultRequest { name, description });

    let response = client
        .create_new_vault(request)
        .await
        .map_err(|err| {
            CommandResponse::new(())
                .set_message(err.message())
                .set_status(CommandResponseStatus::Error)
        })?
        .into_inner();

    Ok(CommandResponse::new(json!({
        "data": response
    })))
}

#[derive(Debug, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct CreateVaultRequest {
    pub name: String,
    pub description: String,
}

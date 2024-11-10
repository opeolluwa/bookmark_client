use std::str::FromStr;

use crate::helpers::authorization::extract_token;
use crate::helpers::ipc_manager::{CommandResponse, CommandResponseStatus, CommandResult};
use tauri::Runtime;
use tonic::metadata::MetadataValue;
use tonic::Request;
use vault_grpc::client_stub::vault::{vault_manager_client::VaultManagerClient, NewVaultRequest};
use vault_grpc::client_stub::vault::{ListVaultsRequest, ListVaultsResponse, NewVaultResponse};

#[tauri::command]
pub async fn create_vault(payload: NewVaultRequest) -> CommandResult<NewVaultResponse> {
    let Some(mut client) = VaultManagerClient::connect("http://127.0.0.1:50051")
        .await
        .ok()
    else {
        return Err(CommandResponse::new(())
            .set_message("Internal server error")
            .set_status(CommandResponseStatus::Error));
    };

    let request = Request::new(payload);
    let response = client
        .create_new_vault(request)
        .await
        .map_err(|err| {
            CommandResponse::new(())
                .set_message(err.message())
                .set_status(CommandResponseStatus::Error)
        })?
        .into_inner();

    Ok(CommandResponse::new(response))
}

#[tauri::command]
pub async fn get_all_vaults<R: Runtime>(
    app: tauri::AppHandle<R>,
    payload: ListVaultsRequest,
) -> CommandResult<ListVaultsResponse> {
    let token = extract_token(&app);
    println!("{:#?}", token.clone().unwrap()["value"].take());
    let Some(mut client) = VaultManagerClient::connect("http://127.0.0.1:50051")
        .await
        .ok()
    else {
        return Err(CommandResponse::new(())
            .set_message("Internal server error")
            .set_status(CommandResponseStatus::Error));
    };

    let mut request = Request::new(payload);
    let token = MetadataValue::from_str( "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2VtYWlsIjoiYWRlb3llQHlhaG9vLmNvbSIsInVzZXJfaWQiOiJjMjI3NTdiNC03YjNjLTQ4NDUtYTcwMy05YzRjNDQ0OGM2ZjQiLCJleHAiOjIwMDAwMDAwMDB9.TgvSelgkIitziyzTl-1th-u0gATdcG2GxvICe77O5Uc").unwrap();
    request.metadata_mut().insert("authorization", token);

    let response = client
        .list_vaults(request)
        .await
        .map_err(|err| {
            CommandResponse::new(())
                .set_message(err.message())
                .set_status(CommandResponseStatus::Error)
        })?
        .into_inner();

    Ok(CommandResponse::new(response))
}

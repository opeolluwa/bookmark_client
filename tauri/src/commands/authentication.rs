use crate::api_request::RequestEndpoint;
use bookmark_components::forms::sign_up::SignUpFormData;
use bookmark_grpc_codegen::client_stub::authentication::SignUpResponse;
use ehttp::{fetch_async, Request};
use tauri_wasm_bindgen::ipc_response::{
    IpcResponseError, IpcResponseSuccess, TauriCommandResponse,
};

#[tauri::command]
pub async fn sign_up(payload: SignUpFormData) -> TauriCommandResponse<SignUpResponse> {
    let request = Request::json(RequestEndpoint::new("users/register"), &payload).unwrap();
    let response = fetch_async(request).await.unwrap();
    let status_code = response.status;
    if status_code != 201 {
        return Err(IpcResponseError::build("invalid status code"));
    }
    let response = response.json::<SignUpResponse>().unwrap();
    Ok(IpcResponseSuccess::new(response))
}

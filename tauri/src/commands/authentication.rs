// use bookmark_grpc_codegen::client_stub::authentication::{SignUpRequest, SignUpResponse};
// use tauri_plugin_http::reqwest;

// use crate::{
//     config::{request_endpoint, CONFIG},
//     ipc_manager::{CommandResponse, CommandResult},
// };

// #[tauri::command]
// async fn register(payload: SignUpRequest) -> CommandResult<SignUpResponse> {
//     let request_endpoint = request_endpoint("/users/register");
//     let response = reqwest::get(request_endpoint)
//         .await
//         .map_err(|error| CommandResponse::new(error))?;
//     todo!()
// }

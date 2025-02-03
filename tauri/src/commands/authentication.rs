<<<<<<< HEAD
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
=======
use crate::api_request::{endpoints, RequestEndpoint};
use bookmark_components::forms::{
    login::LoginFormData,
    sign_up::{RegisterFormData, SignUpResponse},
};

#[tauri::command]
pub async fn login(payload: LoginFormData) {
    println!("{:?}", payload);
    // let client = reqwest::Client::new();
    // let response = client
    //     .post(RequestEndpoint::new(endpoints::LOG_IN_END_POINT))
    //     .json(&payload)
    //     .send()
    //     .await
    //     .map_err(|error| SignUpError {
    //         message: error.to_string(),
    //         success: false,
    //     })?;

    // let response = response
    //     .json::<SignUpResponse>()
    //     .await
    //     .map_err(|error| SignUpResponse {
    //         message: error.to_string(),
    //         success: false,
    //     })?;

    // Ok(SignUpResponse {
    //     message: response.message,
    //     success: response.success,
    // })
}
>>>>>>> c075b4af20325c71e73098574c5a918bee903c38

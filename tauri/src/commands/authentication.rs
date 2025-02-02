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

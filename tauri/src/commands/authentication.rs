use bookmark_components::forms::sign_up::SignUpFormData;
use bookmark_grpc_codegen::client_stub::authentication::SignUpResponse;
use ehttp::{fetch, Request, Response, Result};

use crate::api_request::RequestEndpoint;

#[tauri::command]
pub async fn sign_up(payload: SignUpFormData) {
    let request = Request::json(RequestEndpoint::new("users/register"), &payload)
        .expect("error constructing request");

    fetch(request, move |result: Result<Response>| {
        let response = result
            .expect("error parsing API response")
            .json::<SignUpResponse>();

        println!("Status code: {:?}", response);
    });
}

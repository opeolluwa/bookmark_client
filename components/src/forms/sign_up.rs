use bookmark_ui_errors::api_request_errors::ApiRequestError;
use gloo_net::http::Method;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    endpoints::{self},
    response::FormResponse,
    RequestEndpoint,
};

pub type SignUpResponse = FormResponse<String>;
#[derive(Debug, Serialize, Deserialize, Default, Validate)]
pub struct RegisterFormData {
    #[validate(length(min = 1, message = "first name cannot be empty"))]
    first_name: String,
    #[validate(length(min = 1, message = "last name cannot be empty"))]
    last_name: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters "))]
    password: String,
}

impl RegisterFormData {
    pub fn new(first_name: String, last_name: String, email: String, password: String) -> Self {
        Self {
            first_name,
            last_name,
            email,
            password,
        }
    }

    pub async fn submit(&self) -> Result<SignUpResponse, ApiRequestError> {
        let request_method = Method::POST;
        let request_endpoint = RequestEndpoint::new(endpoints::SIGN_UP_END_POINT);

        let response = gloo_net::http::RequestBuilder::new(&request_endpoint)
            .method(request_method)
            .header("Access-Control-Allow-Origin", "no-cors")
            .json(&self)
            .map_err(|error| {
                log::error!(
                    "Failed to construct request due to error {}",
                    error.to_string()
                );
                ApiRequestError::ProcessError("Failed to construct request".to_string())
            })?
            .send()
            .await
            .map_err(|error| {
                log::error!("Failed to send request due to error {}", error.to_string());
                ApiRequestError::ProcessError(error.to_string())
            })?;

        let response_status = response.status();
        let response_body = response
            .json::<SignUpResponse>()
            .await
            .map_err(|error| ApiRequestError::ProcessError(error.to_string()))?;

        match response_status {
            201 => Ok(response_body),
            _ => Err(ApiRequestError::ProcessError(
                "Failed to sign up".to_string(),
            )),
        }
    }
}

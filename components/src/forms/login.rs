use bookmark_ui_errors::api_request_errors::ApiRequestError;
use gloo_net::http::Method;
use serde::{Deserialize, Serialize};

use super::{endpoints, FormResponse, RequestEndpoint};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginFormData {
    pub email: String,
    pub password: String,
}

impl LoginFormData {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginResponse {
    pub token: String,
}

impl LoginFormData {
    pub async fn submit(&self) -> Result<FormResponse<LoginResponse>, ApiRequestError> {
        let request_method = Method::POST;
        let request_endpoint = RequestEndpoint::new(endpoints::LOG_IN_END_POINT);

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
        match response_status {
            200 => {
                let response_body = response
                    .json::<FormResponse<LoginResponse>>()
                    .await
                    .map_err(|error| {
                        log::error!(
                            "Failed to parse response body due to error {}",
                            error.to_string()
                        );
                        ApiRequestError::ProcessError("Failed to parse response body".to_string())
                    })?;
                Ok(response_body)
            }
            _ => {
                log::error!(
                    "Failed to sign in due to error {}",
                    response_status.to_string()
                );
                return Err(ApiRequestError::ProcessError(
                    "Failed to sign in".to_string(),
                ));
            }
        }
    }
}

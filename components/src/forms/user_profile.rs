use bookmark_ui_errors::api_request_errors::ApiRequestError;
use gloo_net::http::Method;
use serde::{Deserialize, Serialize};

use super::{endpoints, FormResponse, RequestEndpoint};

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Clone)]
pub struct UserProfile {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub bio: String,
    pub profile_picture: String,
}

impl UserProfile {
    pub async fn fetch(
        bearer_token: &String,
    ) -> Result<FormResponse<UserProfile>, ApiRequestError> {
        let request_method = Method::GET;
        let request_endpoint = RequestEndpoint::new(endpoints::USER_PROFILE_END_POINT);

        let response = gloo_net::http::RequestBuilder::new(&request_endpoint)
            .method(request_method)
            // .header("Access-Control-Allow-Origin", "no-cors")
            .header("Authorization", &format!("Bearer {bearer_token}"))
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
                    .json::<FormResponse<UserProfile>>()
                    .await
                    .map_err(|error| ApiRequestError::ProcessError(error.to_string()))?;
                Ok(response_body)
            }
            _ => Err(ApiRequestError::ProcessError(
                "error fetching user profile".into(),
            )),
        }
    }
}

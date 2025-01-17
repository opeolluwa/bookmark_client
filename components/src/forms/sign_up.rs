use async_trait::async_trait;
use gloo_net::http::Method;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use validator::Validate;

use super::{
    endpoints::{self, Endpoint},
    FormResponse, RequestEndpoint, Response, SubmitForm,
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SignUpResponse {
    pub message: String,
    pub success: bool,
}

pub type SignUpError = SignUpResponse;

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
}

// #[async_trait]
// impl SubmitForm for RegisterFormData {
//     async fn submit<SignUpResponse>(&self, endpoint: Endpoint) -> FormResponse<SignUpResponse> {
//         let request_method = Method::POST;
//         let request_endpoint = RequestEndpoint::new(endpoints::LOG_IN_END_POINT);

//         let response = gloo_net::http::RequestBuilder::new(&request_endpoint)
//             .method(request_method)
//             .header("Access-Control-Allow-Origin", "no-cors")
//             .json(self)
//             .unwrap()
//             .send()
//             .await
//             .map_err(|_| Response {
//                 status: super::ResponseStatus::Failed,
//                 body: (),
//             })?;

//         let response_body = response.json::<SignUpResponse>().await.unwrap();
//         todo!()
//     }
// }

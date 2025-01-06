use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SignUpResponse {
    pub message: String,
    pub success: bool,
}

pub type SignUpError = SignUpResponse;

#[derive(Debug, Serialize, Deserialize, Default, Validate)]
pub struct SignUpFormData {
    #[validate(length(min = 1, message = "first name cannot be empty"))]
    first_name: String,
    #[validate(length(min = 1, message = "last name cannot be empty"))]
    last_name: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters "))]
    password: String,
}

impl SignUpFormData {
    pub fn new(first_name: String, last_name: String, email: String, password: String) -> Self {
        Self {
            first_name,
            last_name,
            email,
            password,
        }
    }
}

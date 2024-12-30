use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpResponse {
    pub message: String,
    pub status: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpFormData {
    first_name: String,
    last_name: String,
    email: String,
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

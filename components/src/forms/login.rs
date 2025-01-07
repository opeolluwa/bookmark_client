use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub pid: String,
    pub name: String,
    pub is_verified: bool,
}

pub type LoginError = LoginResponse;

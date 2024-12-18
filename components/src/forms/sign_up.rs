use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpFormData {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

impl SignUpFormData {
    pub fn new(
        first_name: &'static str,
        last_name: &'static str,
        email: &'static str,
        password: &'static str,
    ) -> Self {
        Self {
            first_name: first_name.into(),
            last_name: last_name.into(),
            email: email.into(),
            password: password.into(),
        }
    }
}

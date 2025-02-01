use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FormResponse<T> {
    pub message: String,
    pub body: Option<T>,
}

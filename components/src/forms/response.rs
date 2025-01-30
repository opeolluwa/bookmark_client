use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FormResponse<T> {
    message: String,
    body: Option<T>,
}

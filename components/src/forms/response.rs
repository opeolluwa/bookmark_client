use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub status: ResponseStatus,
    pub body: T,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum ResponseStatus {
    Timeout,
    #[default]
    Success,
    Failed,
}

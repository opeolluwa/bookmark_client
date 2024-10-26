use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct IpcResponse<T> {
    pub body: T,
    pub message: String,
    pub status: IpcResponseStatus,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum IpcResponseStatus {
    Success,
    Error,
    Aborted,
}

impl ToString for IpcResponseStatus {
    fn to_string(&self) -> String {
        match self {
            IpcResponseStatus::Success => "success".to_string(),
            IpcResponseStatus::Error => "error".to_string(),
            IpcResponseStatus::Aborted => "aborted".to_string(),
        }
    }
}

impl<T> IpcResponse<T> {
    pub fn new(body: T, message: &str, status: IpcResponseStatus) -> Self {
        Self {
            body,
            message: message.to_string(),
            status,
        }
    }
}

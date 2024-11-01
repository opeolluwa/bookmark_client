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

pub trait IpcResponseError<T = ()> {
    fn error(message: &str) -> Self;
}

pub trait IpcResponseSuccess<T> {
    fn success(message: &str, data: T) -> Self;
}

impl<T> IpcResponse<T> {
    pub fn new(body: T, message: &str, status: IpcResponseStatus) -> Self {
        Self {
            body,
            message: message.to_string(),
            status,
        }
    }

    pub fn success(message: &str, body: T) -> Self {
        Self {
            body,
            message: message.to_string(),
            status: IpcResponseStatus::Success,
        }
    }
}

impl IpcResponseError for IpcResponse<()> {
    fn error(message: &str) -> Self {
        Self {
            body: (),
            message: message.to_string(),
            status: IpcResponseStatus::Error,
        }
    }
}

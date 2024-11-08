use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::ipc::{InvokeResponseBody, IpcResponse};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS, Clone, Default)]
#[ts(export)]
pub struct CommandResponse {
    pub body: Option<Value>,
    pub message: String,
    pub status: CommandResponseStatus,
}

// impl<T: Serialize> tauri::ipc::CommandResponse for CommandResponse<T> {
//     fn body(self) -> tauri::Result<tauri::ipc::InvokeResponseBody> {
//         self.body
//     }
// }
#[derive(Debug, Serialize, Deserialize, TS, Clone)]
#[ts(export)]
pub enum CommandResponseStatus {
    Success,
    Error,
    Aborted,
}

impl Default for CommandResponseStatus {
    fn default() -> Self {
        Self::Success
    }
}

impl ToString for CommandResponseStatus {
    fn to_string(&self) -> String {
        match self {
            CommandResponseStatus::Success => "success".to_string(),
            CommandResponseStatus::Error => "error".to_string(),
            CommandResponseStatus::Aborted => "aborted".to_string(),
        }
    }
}

impl<T: Clone> CommandResponse<T>
where
    CommandResponse<T>: std::default::Default,
{
    pub fn new(body: T) -> Self {
        Self {
            body: Some(body),
            ..Default::default()
        }
    }
    pub fn set_status(&self, status: CommandResponseStatus) -> Self {
        Self {
            status,
            body: self.body.clone(),
            message: self.message.to_owned(),
        }
    }
    pub fn set_message(&self, message: &str) -> Self {
        Self {
            body: Some(self.body.clone().unwrap().to_owned()),
            message: message.to_string(),
            status: self.status.to_owned(),
        }
    }
}

// impl<T: std::clone::Clone> IpcResponse for CommandResponse<T> {
//     fn body(self) -> tauri::Result<tauri::ipc::InvokeResponseBody> {
//         let res = InvokeResponseBody::from(self);
//         Ok(res)
//     }
// }

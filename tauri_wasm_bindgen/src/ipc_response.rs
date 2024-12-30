use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub type IpcResponseError = IpcResponse<()>;
pub type IpcResponseSuccess<T> = IpcResponse<T>;
pub type TauriCommandResponse<T> = Result<IpcResponseSuccess<T>, IpcResponseError>;

impl<T: Serialize> IpcResponseSuccess<T> {
    pub fn new(body: T) -> Self {
        Self {
            status: IpcResponseStatus::Success,
            body: Some(body),
            ..Default::default()
        }
    }
}

impl IpcResponseError {
    pub fn build(message: &str) -> Self {
        Self {
            status: IpcResponseStatus::Failed,
            message: message.into(),
            body: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IpcResponse<T>
where
    T: Serialize,
{
    pub message: String,
    pub status: IpcResponseStatus,
    pub body: Option<T>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum IpcResponseStatus {
    #[default]
    Success,
    Failed,
}

impl<T: Serialize> Default for IpcResponse<T> {
    fn default() -> Self {
        Self {
            message: Default::default(),
            status: Default::default(),
            body: None::<T>,
        }
    }
}

impl std::fmt::Display for IpcResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_value = match self {
            IpcResponseStatus::Failed => "failed".to_string(),
            IpcResponseStatus::Success => "success".to_string(),
        };
        write!(f, "{}", string_value)
    }
}

impl FromStr for IpcResponseStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "failed" => Ok(IpcResponseStatus::Failed),
            "success" => Ok(IpcResponseStatus::Success),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ipc_response_status_enum_case_conversion_failed_arm() {
        assert_eq!(
            IpcResponseStatus::Failed.to_string(),
            "FAILED".to_lowercase()
        );
    }

    #[test]
    fn failed_status_from_str() {
        assert_eq!(
            IpcResponseStatus::from_str("failed  ").ok(),
            Some(IpcResponseStatus::Failed)
        )
    }

    #[test]
    fn success_status_from_str() {
        assert_eq!(
            IpcResponseStatus::from_str("  success ").ok(),
            Some(IpcResponseStatus::Success)
        )
    }
    #[test]
    fn ipc_response_status_enum_case_conversion_success_arm() {
        assert_eq!(
            IpcResponseStatus::Success.to_string(),
            "SUCCESS".to_lowercase()
        );
    }
    #[test]
    fn ipc_response_status_enum_test_success_arm() {
        assert_eq!(
            IpcResponseStatus::Success.to_string(),
            "success".to_string()
        );
    }

    #[test]
    fn ipc_response_status_enum_test_failed_arm() {
        assert_eq!(IpcResponseStatus::Failed.to_string(), "failed".to_string());
    }

    #[test]
    fn test_icp_success_message() {
        #[derive(Serialize)]
        struct SampleResponse {
            name: String,
        }

        let ipc_resp = IpcResponseSuccess::new(SampleResponse {
            name: "test".to_string(),
        });
        assert_eq!(ipc_resp.status.to_string(), "success")
    }

    #[test]
    fn test_ipc_error_response() {
        let ipc_resp: IpcResponseError = IpcResponseError::build("error message");

        assert_eq!(ipc_resp.message, "error message".to_string())
    }
}

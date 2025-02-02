use codee::string::JsonSerdeCodec;
use leptos::prelude::{Get, Set};
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

const INSTALLATION_STATE: &str = "installation_state";
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InstallationStatus {
    pub is_installed: bool,
}

impl InstallationStatus {
    pub fn read_state() -> Self {
        let (state, _, _) =
            use_local_storage::<InstallationStatus, JsonSerdeCodec>(INSTALLATION_STATE);

        state.get()
    }

    pub fn set_installed() {
        let (_, set_state, _) =
            use_local_storage::<InstallationStatus, JsonSerdeCodec>(INSTALLATION_STATE);

        set_state.set(InstallationStatus { is_installed: true });
    }
}

impl Default for InstallationStatus {
    fn default() -> Self {
        InstallationStatus {
            is_installed: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_installation_status_default() {
        let installation_status = InstallationStatus::default();
        assert_eq!(installation_status.is_installed, false);
    }
}

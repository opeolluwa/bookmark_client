use std::{fmt::Display, str::FromStr};

use codee::string::FromToStringCodec;
use leptos::prelude::{Get, Set};
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

const ACCESS_TOKEN: &str = "access_token";
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AccessToken(String);

impl AccessToken {
    pub fn fetch() -> Self {
        let (state, _, _) = use_local_storage::<AccessToken, FromToStringCodec>(ACCESS_TOKEN);

        state.get()
    }

    pub fn set(token: String) {
        let (_, set_state, _) = use_local_storage::<AccessToken, FromToStringCodec>(ACCESS_TOKEN);

        set_state.set(AccessToken::from(token));
    }

    pub fn set_empty() {
        let (_, set_state, _) = use_local_storage::<AccessToken, FromToStringCodec>(ACCESS_TOKEN);

        set_state.set(AccessToken::default());
    }
}

impl Default for AccessToken {
    fn default() -> Self {
        AccessToken(String::new())
    }
}

impl From<String> for AccessToken {
    fn from(value: String) -> Self {
        AccessToken(value)
    }
}

impl From<AccessToken> for String {
    fn from(value: AccessToken) -> String {
        value.0
    }
}

impl FromStr for AccessToken {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AccessToken(s.to_string()))
    }
}

impl Display for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_access_token_default() {
        let access_token = AccessToken::default();
        assert_eq!(access_token.0, "");
    }
}

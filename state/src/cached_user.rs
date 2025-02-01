use codee::string::JsonSerdeCodec;
use leptos::prelude::{Get, Set};
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

use bookmark_components::forms::user_profile::UserProfile;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CachedUser {
    pub user: Option<UserProfile>,
}

impl Default for CachedUser {
    fn default() -> Self {
        Self { user: None }
    }
}

impl CachedUser {
    pub fn read_state() -> Self {
        let (state, _, _) = use_local_storage::<CachedUser, JsonSerdeCodec>("cached_user");

        state.get()
    }

    pub fn set_user(user: UserProfile) {
        let (_, set_state, _) = use_local_storage::<CachedUser, JsonSerdeCodec>("cached_user");

        set_state.set(CachedUser { user: Some(user) });
    }
}

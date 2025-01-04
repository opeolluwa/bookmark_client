use codee::string::JsonSerdeCodec;
use leptos::prelude::{Get, Set};
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CachesUser {
    pub user: Option<User>,
}

impl Default for CachesUser {
    fn default() -> Self {
        Self { user: None }
    }
}

impl CachesUser {
    pub fn read_state() -> Self {
        let (state, _, _) = use_local_storage::<CachesUser, JsonSerdeCodec>("cached_user");

        state.get()
    }

    pub fn set_user(user: User) {
        let (_, set_state, _) = use_local_storage::<CachesUser, JsonSerdeCodec>("cached_user");

        set_state.set(CachesUser { user: Some(user) });
    }
}

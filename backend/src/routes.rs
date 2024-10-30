use axum::routing::{get, post, Router};

use crate::{
    handler::{
        user_information::{login, register_user},
        root::{base, health_check_handler},
        vault::some_protected_resources,
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(base))
        .route("/health", get(health_check_handler))
        .nest("/users", user_route())
        .nest("/api/v1", protected_routes())
}

pub fn user_route() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login))
}

pub fn protected_routes() -> Router<AppState> {
    Router::new().route("/protected", get(some_protected_resources))
}

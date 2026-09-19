use axum::Router;
use axum::routing::{put};
use crate::handlers;
use crate::models::pool::AppState;

pub fn users_update(_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/users/update/username", put(handlers::users_update_username))
        .route("/users/update/password", put(handlers::users_update_password))
        .route("/users/update/nickname", put(handlers::users_update_nickname))
}
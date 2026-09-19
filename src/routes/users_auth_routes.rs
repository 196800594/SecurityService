use axum::Router;
use axum::routing::{post};
use crate::handlers;
use crate::models::pool::AppState;

pub fn pw_auth(_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/users/pw-auth/login", post(handlers::users_pw_auth_login))
        .route("/users/pw-auth/register", post(handlers::users_pw_auth_register))
}
mod users_auth_routes;
mod users_update_routes;

pub use users_auth_routes::*;

use axum::Router;
use crate::models::pool::AppState;
use crate::routes::users_update_routes::users_update;

pub fn app_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(pw_auth(state.clone()))
        .merge(users_update(state.clone()))
}

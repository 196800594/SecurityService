mod users_auth_routes;

pub use users_auth_routes::*;

use axum::Router;
use crate::models::pool::AppState;

pub fn app_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(sms_auth(state))
}

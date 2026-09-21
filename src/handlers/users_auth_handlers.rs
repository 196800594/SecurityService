use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use rust_i18n::t;
use crate::models::{Response, LoginRequest, RegisterRequest};
use crate::models::pool::AppState;
use crate::services;
use crate::utils::Lang;

pub async fn  users_pw_auth_login(State(state): State<AppState>, Lang(lang): Lang, Json(request): Json<LoginRequest>) -> impl IntoResponse {
    let pool = state.pool.clone();
    let result = services::users_pw_auth_login(&pool, &lang, request).await;
    match result {
        Ok(result) => {
            let message = t!("auth.success.login", locale=&lang).to_string();
            let response = Response::new(message, Some(result));
            (StatusCode::OK, Json(response)).into_response()
        },
        Err(error) => { error.into_response() }
    }
}

pub async fn users_pw_auth_register(State(state): State<AppState>, Lang(lang): Lang, Json(request): Json<RegisterRequest>) -> impl IntoResponse {
    let pool = state.pool.clone();
    let result = services::users_pw_auth_register(&pool, &lang, request).await;
    match result { 
        Ok(result) => {
            let message = t!("auth.success.register", locale=&lang).to_string();
            let response = Response::new(message, Some(result));
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(error) => { error.into_response() }
    }
}

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::models::{Response, LoginRequest, RegisterRequest};
use crate::models::pool::AppState;
use crate::services;

pub async fn  users_pw_auth_login(State(state): State<AppState>, Json(request): Json<LoginRequest>) -> impl IntoResponse {
    let pool = state.pool.clone();
    let result = services::users_pw_auth_login(&pool, request).await;
    match result {
        Ok(result) => {
            let message = "登录成功".to_string();
            let response = Response::new(message, Some(result));
            (StatusCode::OK, Json(response)).into_response()
        },
        Err(error) => { error.into_response() }
    }
}

pub async fn users_pw_auth_register(State(state): State<AppState>, Json(request): Json<RegisterRequest>) -> impl IntoResponse {
    let pool = state.pool.clone();
    let result = services::users_pw_auth_register(&pool, request).await;
    match result { 
        Ok(result) => {
            let message = "注册成功".to_string();
            let response = Response::new(message, Some(result));
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(error) => { error.into_response() }
    }
}

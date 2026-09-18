use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use crate::models::{Response, LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
use crate::models::pool::AppState;
use crate::services;
use crate::utils;

pub async fn  users_pw_auth_login(State(state): State<AppState>, Json(request): Json<LoginRequest>) -> (StatusCode, Json<Response<LoginResponse>>) {
    let pool = state.pool.clone();
    let (result, data) = services::users_pw_auth_login(&pool, request).await;
    utils::result_handle(result, data)
}

pub async fn users_pw_auth_register(State(state): State<AppState>, Json(request): Json<RegisterRequest>) -> (StatusCode, Json<Response<RegisterResponse>>) {
    let pool = state.pool.clone();
    let (result, data) = services::users_pw_auth_register(&pool, request).await;
    utils::result_handle(result, data)
}

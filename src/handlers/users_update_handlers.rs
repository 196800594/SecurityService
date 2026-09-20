use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::models::pool::AppState;
use crate::{services};
use crate::models::{Response, UpdateNicknameRequest, UpdatePasswordRequest, UpdateUsernameRequest};

pub async fn users_update_username(State(state): State<AppState>, Json(request): Json<UpdateUsernameRequest>) -> impl IntoResponse {
    let pool = state.pool.clone();
    let result = services::users_update_username(&pool, request).await;
    match result {
        Ok(result) => {
            let message = "用户名更新成功".to_string();
            let response = Response::new(message, Some(result));
            (StatusCode::OK, Json(response)).into_response()
        },
        Err(error) => { error.into_response() }
    }
}

pub async fn users_update_password(State(state): State<AppState>, Json(request): Json<UpdatePasswordRequest>) -> impl IntoResponse  {
    let pool = state.pool.clone();
    let result = services::users_update_password(&pool, request).await;
    match result {
        Ok(result) => {
            let message = "密码更新成功".to_string();
            let response = Response::new(message, Some(result));
            (StatusCode::OK, Json(response)).into_response()
        },
        Err(error) => { error.into_response() }
    }

}

pub async fn users_update_nickname(State(state): State<AppState>, Json(request): Json<UpdateNicknameRequest>) -> impl IntoResponse  {
    let pool = state.pool.clone();
    let result = services::users_update_nickname(&pool, request).await;
    match result {
        Ok(result) => {
            let message = "昵称更新成功".to_string();
            let response = Response::new(message, Some(result));
            (StatusCode::OK, Json(response)).into_response()
        },
        Err(error) => { error.into_response() }
    }
}
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::models::pool::AppState;
use crate::{services};
use crate::models::{Response, UpdateNicknameRequest, UpdatePasswordRequest, UpdateUsernameRequest};
use crate::utils::Lang;

pub async fn users_update_username(State(state): State<AppState>, Lang(lang): Lang, Json(request): Json<UpdateUsernameRequest>) -> impl IntoResponse {
    let pool = state.pool.clone();
    let result = services::users_update_username(&pool, &lang, request).await;
    match result {
        Ok(result) => {
            let message = t!("update.success.username", locale=&lang).to_string();
            let response = Response::new(message, Some(result));
            (StatusCode::OK, Json(response)).into_response()
        },
        Err(error) => { error.into_response() }
    }
}

pub async fn users_update_password(State(state): State<AppState>, Lang(lang): Lang, Json(request): Json<UpdatePasswordRequest>) -> impl IntoResponse  {
    let pool = state.pool.clone();
    let result = services::users_update_password(&pool, &lang, request).await;
    match result {
        Ok(result) => {
            let message = t!("update.success.password", locale=&lang).to_string();
            let response = Response::new(message, Some(result));
            (StatusCode::OK, Json(response)).into_response()
        },
        Err(error) => { error.into_response() }
    }

}

pub async fn users_update_nickname(State(state): State<AppState>, Lang(lang): Lang, Json(request): Json<UpdateNicknameRequest>) -> impl IntoResponse  {
    let pool = state.pool.clone();
    let result = services::users_update_nickname(&pool, &lang, request).await;
    match result {
        Ok(result) => {
            let message = t!("update.success.nickname", locale=&lang).to_string();
            let response = Response::new(message, Some(result));
            (StatusCode::OK, Json(response)).into_response()
        },
        Err(error) => { error.into_response() }
    }
}
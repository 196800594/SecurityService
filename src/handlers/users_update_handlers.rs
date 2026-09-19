use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use crate::models::pool::AppState;
use crate::models::{ Response, UpdateRequest, UpdateResponse };
use crate::{services, utils};

pub async fn users_update_username(State(state): State<AppState>, Json(request): Json<UpdateRequest>) -> (StatusCode, Json<Response<UpdateResponse>>){
    let pool = state.pool.clone();
    let (result, data) = services::users_update_username(&pool, request).await;
    utils::result_handle(result, data)
}

pub async fn users_update_password(State(state): State<AppState>, Json(request): Json<UpdateRequest>) -> (StatusCode, Json<Response<UpdateResponse>>) {
    let pool = state.pool.clone();
    let (result, data) = services::users_update_password(&pool, request).await;
    utils::result_handle(result, data)
}

pub async fn users_update_nickname(State(state): State<AppState>, Json(request): Json<UpdateRequest>) -> (StatusCode, Json<Response<UpdateResponse>>) {
    let pool = state.pool.clone();
    let (result, data) = services::users_update_nickname(&pool, request).await;
    utils::result_handle(result, data)

}
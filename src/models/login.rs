use jwt_simple::prelude::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(FromRow, Serialize)]
pub struct LoginResponse {
    pub token: String,
}
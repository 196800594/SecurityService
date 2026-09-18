use jwt_simple::prelude::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(FromRow, Serialize)]
pub struct LoginResponse {
    pub token: String,
}
impl LoginResponse {
    pub fn new(token: String) -> LoginResponse {
        LoginResponse { token }
    }
}
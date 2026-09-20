use serde::{Deserialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub nickname: String,
}
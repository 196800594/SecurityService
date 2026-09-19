use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRequest {
    pub token: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub nickname: Option<String>,
}

#[derive(Serialize)]
pub struct UpdateResponse {}
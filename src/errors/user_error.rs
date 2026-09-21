use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use thiserror::Error;
use crate::models;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("登录失败，请检查用户名或密码")]
    Unauthorized,
    #[error("用户名已存在")]
    UsernameAlreadyExists,
    #[error("用户不存在")]
    UserNotFound,
    #[error("登录信息过期，请重新登录")]
    TokenExpired,
    #[error("服务器内部错误")]
    InternalServerError,
}
impl UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserError::Unauthorized => StatusCode::UNAUTHORIZED,
            UserError::UsernameAlreadyExists => StatusCode::CONFLICT,
            UserError::UserNotFound => StatusCode::NOT_FOUND,
            UserError::TokenExpired => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl IntoResponse for UserError {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code();
        let error_response: models::response::Response<()> = models::Response::new(self.to_string(), None);
        (status_code, Json(error_response)).into_response()
    }
}
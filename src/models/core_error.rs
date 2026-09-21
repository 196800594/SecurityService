use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use thiserror::Error;
use crate::models;

#[derive(Error, Debug)]
pub enum CoreError {
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
    #[error("哈希校验失败")]
    HashVerify,
    #[error("哈希解析失败")]
    HashAnalysis,
    #[error("哈希生成失败")]
    HashGenerate,
    #[error("令牌校验失败>> {0}")]
    JwtVerify(#[source] jwt_simple::Error),
    #[error("令牌生成失败>> {0}")]
    JwtGenerate(#[source] jwt_simple::Error),
    #[error("数据库错误>> {0}")]
    Database(#[from] sqlx::Error),
    #[error("Uuid格式无效>> {0}")]
    InvalidUuid(#[from] uuid::Error),
}
impl CoreError {
    fn status_code(&self) -> StatusCode {
        match self {
            CoreError::Unauthorized => StatusCode::UNAUTHORIZED,
            CoreError::UsernameAlreadyExists => StatusCode::CONFLICT,
            CoreError::HashVerify => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl IntoResponse for CoreError {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code();
        let error_response: models::response::Response<()> = models::Response::new(self.to_string(), None);
        (status_code, Json(error_response)).into_response()
    }
}
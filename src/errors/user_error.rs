use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use thiserror::Error;
use crate::models;
use rust_i18n::t;

#[derive(Error, Debug)]
pub enum UserErrorCode {
    #[error("auth.error.unauthorized")]
    Unauthorized,
    #[error("auth.error.username_already_exists")]
    UsernameAlreadyExists,
    #[error("auth.error.user_not_found")]
    UserNotFound,
    #[error("auth.error.token_expired")]
    TokenExpired,
    #[error("server.error.internal_server_error")]
    InternalServerError,
}
impl UserErrorCode {
    fn status_code(&self) -> StatusCode {
        match self {
            UserErrorCode::Unauthorized => StatusCode::UNAUTHORIZED,
            UserErrorCode::UsernameAlreadyExists => StatusCode::CONFLICT,
            UserErrorCode::UserNotFound => StatusCode::NOT_FOUND,
            UserErrorCode::TokenExpired => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
pub struct UserError {
    error_code: UserErrorCode,
    lang: String,
}
impl UserError {
    pub fn new(error_code: UserErrorCode, lang: &String) -> Self {
        Self { error_code, lang: lang.clone() }
    }
}

impl IntoResponse for UserError {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.error_code.status_code();
        let message_code = self.error_code.to_string();
        let lang = self.lang;
        let message = t!(message_code, locale=&lang).to_string();
        let error_response: models::response::Response<()> = models::Response::new(message, None);
        (status_code, Json(error_response)).into_response()
    }
}
use axum::http::StatusCode;

#[derive(Debug)]
pub enum Error {
    Unauthorized,
    InternalError,
    UsernameAlreadyExists,
}
impl Error {
    pub fn into(self) -> (StatusCode, String) {
        let (status, message) = match self {
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, "登录失败，请检查用户名或密码"),
            Error::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误"),
            Error::UsernameAlreadyExists => (StatusCode::CONFLICT, "用户名已存在"),
        };
        (status, message.to_string())
    }
}
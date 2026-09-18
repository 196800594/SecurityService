use axum::http::StatusCode;

pub enum Success {
    LoginSuccess,
    RegisterSuccess,
}
impl Success {
    pub fn into(self) -> (StatusCode, String) {
        let (status, message) = match self { 
            Success::LoginSuccess => (StatusCode::OK, "登录成功"),
            Success::RegisterSuccess => (StatusCode::CREATED, "注册成功")
        };
        (status, message.to_string())
    }
}
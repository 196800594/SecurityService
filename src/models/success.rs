use axum::http::StatusCode;

pub enum Success {
    LoginSuccess,
    RegisterSuccess,
    UpdateUsernameSuccess,
    UpdatePasswordSuccess,
    UpdateNicknameSuccess,
}
impl Success {
    pub fn into(self) -> (StatusCode, String) {
        let (status, message) = match self { 
            Success::LoginSuccess => (StatusCode::OK, "登录成功"),
            Success::RegisterSuccess => (StatusCode::CREATED, "注册成功"),
            Success::UpdateUsernameSuccess => (StatusCode::OK, "用户名修改成功"),
            Success::UpdatePasswordSuccess => (StatusCode::OK, "密码修改成功"),
            Success::UpdateNicknameSuccess => (StatusCode::OK, "昵称修改成功"),
        };
        (status, message.to_string())
    }
}
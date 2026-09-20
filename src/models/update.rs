use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateUsernameRequest {
    pub token: String,
    pub username: String,

}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdatePasswordRequest {
    pub token: String,
    pub password: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateNicknameRequest {
    pub token: String,
    pub nickname: String,
}
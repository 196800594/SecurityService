use sqlx::MySqlPool;
use crate::models::*;
use crate::utils;
use uuid::Uuid;
use crate::utils::hash_pw;

pub async fn users_pw_auth_login(pool: &MySqlPool, request: LoginRequest) -> (Result<Success, Error>, Option<LoginResponse>) {
    //从Mysql中获取对应username的uuid和哈希密码
    let row = sqlx::query!("SELECT uuid, hash_pw FROM users WHERE username = ?", request.username)
            .fetch_optional(pool)
            .await;
    match row {
        //如果获取成功，则先比较密码和哈希值是否对的上
        Ok(Some(result)) => {
            if (utils::hash_verify(request.password, result.hash_pw)) {
                //将uuid结合时间戳生成 token
                let uuid_bytes: Vec<u8> = result.uuid;
                let uuid = Uuid::from_slice(&uuid_bytes).unwrap();
                let token = utils::token_generator(uuid.to_string());
                let response = LoginResponse::new(token);
                (Ok(Success::LoginSuccess), Some(response))
            }
            else{ (Err(Error::Unauthorized), None) }
        }
        Err(_e) => { (Err(Error::InternalError), None) }
        _ => { (Err(Error::Unauthorized), None) }
    }
}


pub async fn users_pw_auth_register(pool: &MySqlPool, request: RegisterRequest) -> (Result<Success, Error>, Option<RegisterResponse>) {
    //生成唯一 uuid
    let generated_uuid = Uuid::now_v7();
    //将密码转为哈希
    let hash_pw = hash_pw(request.password);
    //验证 username 是否已经存在
    let row = sqlx::query!("INSERT IGNORE INTO users (uuid, username, hash_pw, nickname) VALUES (?, ?, ?, ?)",
    generated_uuid,
    request.username,
    hash_pw,
    request.nickname
    ).execute(pool)
     .await;
    match row {
        Ok(result) => {
            if (result.rows_affected() > 0) {
                (Ok(Success::RegisterSuccess), None)
            }
            else { (Err(Error::UsernameAlreadyExists), None)  }
        }
        Err(_e) => { (Err(Error::InternalError), None) }
    }
}

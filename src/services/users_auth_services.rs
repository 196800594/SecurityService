use sqlx::MySqlPool;
use crate::models::*;
use crate::utils;
use uuid::Uuid;
use crate::utils::{hash_generate, hash_verify};

pub async fn users_pw_auth_login(pool: &MySqlPool, request: LoginRequest) -> Result<LoginResponse, CoreError> {
    //从Mysql中获取对应username的 uuid 和哈希密码
    let row = sqlx::query!("SELECT uuid, hash_pw FROM users WHERE username = ?", request.username)
            .fetch_optional(pool)
            .await
        .map_err(move |e| {
            eprintln!("{}", e);
            CoreError::InternalServerError
        })?;
    //如果获取成功，则先比较密码和哈希值是否对的上
    let record = row.ok_or_else(||CoreError::Unauthorized)?;
    let _hash_verify_result = hash_verify(request.password, record.hash_pw).map_err(|e|{
        eprintln!("{}", e);
        CoreError::Unauthorized
    })?;
    //将uuid结合时间戳生成 token
    let uuid_bytes: Vec<u8> = record.uuid;
    let token = utils::token_generator(uuid_bytes).map_err(|e|{
        eprintln!("{}", e);
        CoreError::InternalServerError
    })?;
    let login_response = LoginResponse { token };
    Ok(login_response)
}


pub async fn users_pw_auth_register(pool: &MySqlPool, request: RegisterRequest) -> Result<(), CoreError> {
    //生成唯一 uuid
    let generated_uuid = Uuid::now_v7();
    //将密码转为哈希
    let hash_pw = hash_generate(request.password).map_err(|e|{
        eprintln!("{}", e);
        CoreError::InternalServerError
    })?;
    //验证 username 是否已经存在
    let row = sqlx::query!("INSERT IGNORE INTO users (uuid, username, hash_pw, nickname) VALUES (?, ?, ?, ?)",
    generated_uuid,
    request.username,
    hash_pw,
    request.nickname
    ).execute(pool)
     .await.map_err(|e| { 
        eprintln!("{}", e);
        CoreError::InternalServerError
    })?;
    if row.rows_affected() == 1 { Ok(()) } else { Err(CoreError::UsernameAlreadyExists) }
}
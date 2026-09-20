use sqlx::MySqlPool;
use crate::models::{CoreError, UpdateNicknameRequest, UpdatePasswordRequest, UpdateUsernameRequest};
use crate::utils;

pub async fn users_update_username(pool: &MySqlPool, request: UpdateUsernameRequest) -> Result<(), CoreError> {
    //解析 token 获取用户的 uuid
    let uuid = utils::token_verify(request.token).map_err(|e| {
        eprintln!("{}", e);
        CoreError::TokenExpired
    })?;
    let row = sqlx::query!("UPDATE users SET username = ? WHERE uuid = ?",
        request.username,
        uuid
        ).execute(pool)
         .await
        .map_err(|e|{
            eprintln!("{}", e);
            CoreError::InternalServerError
    })?;
    if row.rows_affected() == 1 { Ok(()) } else { Err(CoreError::UserNotFound) }
}

pub async fn users_update_password(pool: &MySqlPool, request: UpdatePasswordRequest) -> Result<(), CoreError> {
    //解析 token 获取用户的 uuid
    let uuid = utils::token_verify(request.token).map_err(|e| {
        eprintln!("{}", e);
        CoreError::TokenExpired
    })?;
    let hash_pw = utils::hash_generate(request.password).map_err(|e| {
        eprintln!("{}", e);
        CoreError::InternalServerError
    })?;
    let row = sqlx::query!("UPDATE users SET hash_pw = ? WHERE uuid = ?",
        hash_pw,
        uuid
        ).execute(pool)
        .await
        .map_err(|e|{
            eprintln!("{}", e);
            CoreError::InternalServerError
        })?;
    if row.rows_affected() == 1 { Ok(()) } else { Err(CoreError::UserNotFound) }
}

pub async fn users_update_nickname(pool: &MySqlPool, request: UpdateNicknameRequest) -> Result<(), CoreError> {
    //解析 token 获取用户的 uuid
    let uuid = utils::token_verify(request.token).map_err(|e| {
        eprintln!("{}", e);
        CoreError::TokenExpired
    })?;
    let row = sqlx::query!("UPDATE users SET nickname = ? WHERE uuid = ?",
        request.nickname,
        uuid
        ).execute(pool)
        .await
        .map_err(|e|{
            eprintln!("{}", e);
            CoreError::InternalServerError
        })?;
    if row.rows_affected() == 1 { Ok(()) } else { Err(CoreError::UserNotFound) }
}





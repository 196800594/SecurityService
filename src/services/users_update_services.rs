use sqlx::MySqlPool;
use crate::errors::{UserError, UserErrorCode};
use crate::models::{UpdateNicknameRequest, UpdatePasswordRequest, UpdateUsernameRequest};
use crate::utils;

pub async fn users_update_username(pool: &MySqlPool, lang: &String, request: UpdateUsernameRequest) -> Result<(), UserError> {
    //解析 token 获取用户的 uuid
    let uuid = utils::token_verify(request.token).map_err(|e| {
        eprintln!("{}", e);
        UserError::new(UserErrorCode::TokenExpired, &lang)
    })?;
    let row = sqlx::query!("UPDATE users SET username = ? WHERE uuid = ?",
        request.username,
        uuid
        ).execute(pool)
         .await
        .map_err(|e|{
            eprintln!("{}", e);
            UserError::new(UserErrorCode::InternalServerError, &lang)
    })?;
    if row.rows_affected() == 1 { Ok(()) } else { Err(UserError::new(UserErrorCode::UserNotFound, &lang)) }
}

pub async fn users_update_password(pool: &MySqlPool, lang: &String, request: UpdatePasswordRequest) -> Result<(), UserError> {
    //解析 token 获取用户的 uuid
    let uuid = utils::token_verify(request.token).map_err(|e| {
        eprintln!("{}", e);
        UserError::new(UserErrorCode::TokenExpired, &lang)
    })?;
    let hash_pw = utils::hash_generate(request.password).map_err(|e| {
        eprintln!("{}", e);
        UserError::new(UserErrorCode::InternalServerError, &lang)
    })?;
    let row = sqlx::query!("UPDATE users SET hash_pw = ? WHERE uuid = ?",
        hash_pw,
        uuid
        ).execute(pool)
        .await
        .map_err(|e|{
            eprintln!("{}", e);
            UserError::new(UserErrorCode::InternalServerError, &lang)
        })?;
    if row.rows_affected() == 1 { Ok(()) } else { Err(UserError::new(UserErrorCode::UserNotFound, &lang)) }
}

pub async fn users_update_nickname(pool: &MySqlPool, lang: &String, request: UpdateNicknameRequest) -> Result<(), UserError> {
    //解析 token 获取用户的 uuid
    let uuid = utils::token_verify(request.token).map_err(|e| {
        eprintln!("{}", e);
        UserError::new(UserErrorCode::TokenExpired, &lang)
    })?;
    let row = sqlx::query!("UPDATE users SET nickname = ? WHERE uuid = ?",
        request.nickname,
        uuid
        ).execute(pool)
        .await
        .map_err(|e|{
            eprintln!("{}", e);
            UserError::new(UserErrorCode::InternalServerError, &lang)
        })?;
    if row.rows_affected() == 1 { Ok(()) } else { Err(UserError::new(UserErrorCode::UserNotFound, &lang)) }
}
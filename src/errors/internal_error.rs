use argon2::{password_hash, Argon2};
use jwt_simple::reexports::anyhow;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InternalError {
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
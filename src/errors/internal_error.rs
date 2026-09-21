use argon2::{password_hash, Argon2};
use jwt_simple::reexports::anyhow;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InternalError {
    #[error("Hash verification failed")]
    HashVerify,
    #[error("Hash analysis failed")]
    HashAnalysis,
    #[error("Hash generation failed")]
    HashGenerate,
    #[error("Token verification failed >> {0}")]
    JwtVerify(#[source] jwt_simple::Error),
    #[error("Token generation failed >> {0}")]
    JwtGenerate(#[source] jwt_simple::Error),
    #[error("Database error >> {0}")]
    Database(#[from] sqlx::Error),
    #[error("Invalid UUID format >> {0}")]
    InvalidUuid(#[from] uuid::Error),
}
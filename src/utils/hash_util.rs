use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use crate::models::InternalError;


pub fn hash_generate(password: String) -> Result<String, InternalError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password.as_bytes(), &salt).or(Err(InternalError::HashGenerate))?;
    Ok(hash.to_string())
}

pub fn hash_verify(password: String, hash: String) -> Result<(), InternalError> {
    let parsed_hash = PasswordHash::new(&hash).or(Err(InternalError::HashAnalysis))?;
    let result = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    if result { Ok(()) } else { Err(InternalError::HashVerify) }
}
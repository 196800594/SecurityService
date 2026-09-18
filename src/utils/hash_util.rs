use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

pub fn hash_pw(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn hash_verify(password: String, hash: String) -> bool {
    let parsed_hash = PasswordHash::new(&hash).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash_pw() {
        let hash = hash_pw("password@123456".to_string());
        println!("{}", hash);
    }

    #[test]
    fn test_hash_verify() {
        let hash = hash_verify("password@123456".to_string(), "$argon2id$v=19$m=19456,t=2,p=1$ZSgusWr6Ik6OJgVBi069Gg$/oNzRe2HAlXrhLgGytsDNY2oMF2I/F1eY/GXJMoGw2w".to_string());
        println!("{}", hash);
    }
}
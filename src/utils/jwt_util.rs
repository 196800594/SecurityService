use jwt_simple::prelude::*;
use serde::{Serialize, Deserialize };
use uuid::Uuid;
use crate::models::InternalError;

#[derive(Serialize, Deserialize)]
struct AuthCustom {
    uuid: String,
}

impl AuthCustom {
    pub fn new(uuid: String) -> AuthCustom {
        AuthCustom { uuid }
    }
}

fn get_key() -> HS256Key {
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in environment");
    HS256Key::from_bytes(jwt_secret.as_bytes())
}

fn get_time() -> Duration {
    let jwt_expiration_hours = std::env::var("JWT_EXPIRATION_HOURS")
        .expect("JWT_EXPIRATION_HOURS must be set in environment");
    Duration::from_hours(jwt_expiration_hours.parse().expect("JWT_EXPIRATION_HOURS"))
}

pub fn token_generator(uuid_bytes: Vec<u8>) -> Result<String, InternalError> {
    let key = get_key();
    let uuid = Uuid::from_slice(&uuid_bytes)?;
    let auth_custom = AuthCustom::new(uuid.to_string());
    let claims = Claims::with_custom_claims(auth_custom, get_time());
    let token = key.authenticate(claims).map_err(InternalError::JwtGenerate)?;
    Ok(token)
}

pub fn token_verify(token: String) -> Result<Vec<u8>, InternalError> {
    let key = get_key();
    let claims = key.verify_token::<AuthCustom>(&token, None).map_err(InternalError::JwtVerify)?;
    let uuid = Uuid::parse_str(claims.custom.uuid.as_str())?;
    let uuid_bytes: Vec<u8> = uuid.as_bytes().to_vec();
    Ok(uuid_bytes)
}

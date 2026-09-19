use dotenvy::dotenv;
use jwt_simple::prelude::*;
use serde::{Serialize, Deserialize };
use uuid::Uuid;

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
    Duration::from_hours(jwt_expiration_hours.parse().unwrap())
}

pub fn token_generator(uuid_bytes: Vec<u8>) -> String {
    let key = get_key();
    let uuid = Uuid::from_slice(&uuid_bytes).unwrap();
    let auth_custom = AuthCustom::new(uuid.to_string());
    let claims = Claims::with_custom_claims(auth_custom, get_time());
    key.authenticate(claims).unwrap()
}

pub fn token_verify(token: String) -> Result<Vec<u8>, String> {
    let key = get_key();
    let claims = key.verify_token::<AuthCustom>(&token, None);
    match claims { 
        Ok(claims) => {
            let uuid = Uuid::parse_str(claims.custom.uuid.as_str()).unwrap();
            let uuid_bytes: Vec<u8> = uuid.as_bytes().to_vec();
            Ok(uuid_bytes)
        },
        Err(err) => Err(err.to_string())
    }
}
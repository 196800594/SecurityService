use dotenvy::dotenv;
use jwt_simple::prelude::*;
use serde::{Serialize, Deserialize };

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

pub fn token_generator(uuid: String) -> String {
    let key = get_key();
    let auth_custom = AuthCustom { uuid };
    let claims = Claims::with_custom_claims(auth_custom, get_time());
    key.authenticate(claims).unwrap()
}

pub fn token_verify(token: String) -> Result<String, String> {
    let key = get_key();
    if let Ok(claims) = key.verify_token::<AuthCustom>(&token, None) {
        Ok(claims.custom.uuid)
    } else {
        Err("Token could not be verified".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_token_generator() {
        dotenv().ok();
        let auth_custom = AuthCustom { uuid: "123456".to_string() };
        let token = token_generator(auth_custom.uuid);
        println!("{:?}", token);
    }
    #[test]
    fn test_token_verify() {
        dotenv().ok();
        let result = token_verify("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3ODk2MzA0NjgsImV4cCI6MTc4OTYzNzY2OCwibmJmIjoxNzg5NjMwNDY4LCJ1dWlkIjoiMTIzNDU2In0.cUdknviA2hYLAf4SaNFnaJylUi46aBq3GGM1L062vyg".to_string());
        println!("{:?}", result);
    }
}
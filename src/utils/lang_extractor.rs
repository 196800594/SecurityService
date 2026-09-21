use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

const SUPPORTED_LANGS: &[&str] = &["zh-CN", "en-US"];
const DEFAULT_LANG: &str = "en-US";
const LANG_HEADER: &str = "User-Language";

#[derive(Debug)]
pub struct LangRejection;
impl IntoResponse for LangRejection {
    fn into_response(self) -> Response {
        (
            StatusCode::BAD_REQUEST,
            format!("Unsupported language. Supported: {:?}", SUPPORTED_LANGS),
        )
        .into_response()
    }
}
#[derive(Clone)]
pub struct Lang(pub String);
impl<S> FromRequestParts<S> for Lang
where
    S: Send + Sync,
{
    type Rejection = LangRejection;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user_lang = parts
            .headers
            .get(LANG_HEADER)
            .and_then(|val| val.to_str().ok())
            .unwrap_or(DEFAULT_LANG);

        if SUPPORTED_LANGS.contains(&user_lang) {
            Ok(Lang(user_lang.to_string()))
        } else { Err(LangRejection) }
    }
}
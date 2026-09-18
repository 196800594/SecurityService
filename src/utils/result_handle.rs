use axum::http::StatusCode;
use axum::Json;
use crate::models::{Error, Response, Success};

pub fn result_handle<T>(result: Result<Success, Error>, data: Option<T>) -> (StatusCode, Json<Response<T>>){
    match result {
        Ok(success) => {
            let (status_code, message) = success.into();
            (status_code, Json(Response::new(message, data)))
        }
        Err(error) => {
            let (status_code, message) = error.into();
            (status_code, Json(Response::new(message, None)))
        }
    }
}
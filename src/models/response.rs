use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    message: String,
    data: Option<T>,
}
impl<T> Response<T> {
    pub fn new(message: String, data: Option<T>) -> Response<T> {
        Response { message, data }
    }
}
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,

    TicketDeleteFailIdNotFound { id: u64}
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!(">> {:>12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLE_CLIENT_ERROR").into_response()
    }
}
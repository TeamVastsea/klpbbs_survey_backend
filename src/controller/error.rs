use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum ErrorMessage {
    InvalidParams(String),
    InvalidToken,
    // TokenNotActivated,
    PermissionDenied,
    TooManySubmit,
    NotFound,
    InvalidField { field: String, should_be: String },
    // MissingField,
    Other(String),
    DatabaseError(String),
}

impl IntoResponse for ErrorMessage {
    fn into_response(self) -> Response {
        let builder = Response::builder();

        match self {
            ErrorMessage::InvalidParams(name) => {
                builder.status(StatusCode::BAD_REQUEST).body(format!("Invalid params: {}.", name).into()).unwrap()
            }

            ErrorMessage::InvalidToken => {
                builder.status(StatusCode::UNAUTHORIZED).body("Invalid token.".into()).unwrap()
            }

            // ErrorMessage::TokenNotActivated => {
            //     builder.status(StatusCode::UNAUTHORIZED).body("Token not activated.".into()).unwrap()
            // }

            ErrorMessage::PermissionDenied => {
                builder.status(StatusCode::FORBIDDEN).body("Permission denied.".into()).unwrap()
            }

            ErrorMessage::NotFound => {
                builder.status(StatusCode::NOT_FOUND).body("Not found.".into()).unwrap()
            }

            ErrorMessage::Other(text) => {
                builder.status(StatusCode::INTERNAL_SERVER_ERROR).body(text.into()).unwrap()
            }

            ErrorMessage::TooManySubmit => {
                builder.status(StatusCode::TOO_MANY_REQUESTS).body("Too many submit.".into()).unwrap()
            }

            ErrorMessage::InvalidField { field, should_be } => {
                builder.status(StatusCode::BAD_REQUEST).body(format!("Field {} should be {}.", field, should_be).into()).unwrap()
            }

            // ErrorMessage::MissingField => {
            //     builder.status(StatusCode::BAD_REQUEST).body("Missing field.".into()).unwrap()
            // }

            ErrorMessage::DatabaseError(text) => {
                builder.status(StatusCode::INTERNAL_SERVER_ERROR).body(format!("Database error: {}", text).into()).unwrap()
            }
        }
    }
}
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum ErrorMessage {
    InvalidParams(String),
    InvalidToken,
    TokenNotActivated,
    PermissionDenied,
    NotFound,
    Other(String),
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
            
            ErrorMessage::TokenNotActivated => {
                builder.status(StatusCode::UNAUTHORIZED).body("Token not activated.".into()).unwrap()
            }

            ErrorMessage::PermissionDenied => {
                builder.status(StatusCode::FORBIDDEN).body("Permission denied.".into()).unwrap()
            }

            ErrorMessage::NotFound => {
                builder.status(StatusCode::NOT_FOUND).body("Not found.".into()).unwrap()
            }
            ErrorMessage::Other(text) => {
                builder.status(StatusCode::INTERNAL_SERVER_ERROR).body(text.into()).unwrap()
            }
        }
    }
}
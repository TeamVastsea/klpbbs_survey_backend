use crate::controller::error::ErrorMessage;
use axum::http::HeaderMap;

pub async fn get_token() -> String {
    crate::service::token::create_token().await
}

pub async fn get_user(headers: HeaderMap) -> Result<String, ErrorMessage> {
    let Some(token) = headers.get("token") else { return Err(ErrorMessage::InvalidToken) };

    crate::service::token::get_user_id(token.to_str().unwrap())
        .await
        .map(|data| serde_json::to_string(&data).unwrap())
        .ok_or(ErrorMessage::InvalidToken)
}
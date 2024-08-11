use axum::http::HeaderMap;
use crate::controller::error::ErrorMessage;

pub async fn get_token() -> String {
    crate::service::token::create_token().await
}

pub async fn get_user_id(headers: HeaderMap) -> Result<String, ErrorMessage> {
    let Some(token) = headers.get("token") else { return Err(ErrorMessage::InvalidToken) };
    
    crate::service::token::get_user_id(token.to_str().unwrap())
        .await
        .map(|x| x.to_string())
        .ok_or(ErrorMessage::InvalidToken)
}
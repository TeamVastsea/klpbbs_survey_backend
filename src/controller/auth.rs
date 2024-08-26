use crate::controller::error::ErrorMessage;
use crate::service::admin::AdminTokenInfo;
use crate::service::token::TokenInfo;
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

pub async fn get_admin_token_info(AdminTokenInfo(admin): AdminTokenInfo) -> String {
    serde_json::to_string(&admin).unwrap()
}

pub async fn get_user_info(TokenInfo(user): TokenInfo) -> String {
    serde_json::to_string(&user).unwrap()
}
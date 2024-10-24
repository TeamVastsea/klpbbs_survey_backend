use crate::controller::error::ErrorMessage;
// use crate::controller::oauth::callback::UserData;
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use lazy_static::lazy_static;
use moka::future::Cache;
use rand::Rng;
use std::time::Duration;

lazy_static! {
    static ref TOKEN_CACHE: Cache<String, Option<UserData>> = Cache::builder()
        .time_to_idle(Duration::from_secs(60 * 60 * 24 * 7)) //if the key is not accessed for 7 days, it will be removed
        .build();
}

pub async fn create_token() -> String {
    let token: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    TOKEN_CACHE.insert(token.clone(), None).await;

    token
}

pub async fn activate_token(token: &str, user_id: UserData) {
    TOKEN_CACHE.insert(token.to_string(), Some(user_id)).await;
}

pub async fn get_user_id(token: &str) -> Option<UserData> {
    TOKEN_CACHE.get(token).await.unwrap_or(None)
}

pub struct TokenInfo(pub UserData);

#[async_trait]
impl<S> FromRequestParts<S> for TokenInfo
where S: Send + Sync {
    type Rejection = ErrorMessage;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;
        let token = headers.get("token")
            .ok_or(ErrorMessage::InvalidToken)?
            .to_str()
            .map_err(|_| ErrorMessage::InvalidToken)?;

        let user = get_user_id(token).await
            .ok_or(ErrorMessage::TokenNotActivated)?;

        Ok(TokenInfo(user))
    }
}

#[derive(Clone)]
pub struct UserData {}
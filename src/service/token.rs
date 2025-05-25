use crate::controller::error::ErrorMessage;
use crate::dao::model::user_data::UserData;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use chrono::Utc;
use rand::Rng;
async fn new_token(user: &UserData) -> String {
    let time = Utc::now().timestamp();
    let new_token: String = rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    let new_token = format!("{}-{}", time, new_token);
    user.update_credentials(Some(&new_token)).await.unwrap();

    new_token
}

pub fn validate_token_time(token: &str) -> bool {
    let Some(time) = token.split('-').next() else { return false };
    let Some(time) = time.parse::<i64>().ok() else { return false };
    Utc::now().timestamp() - time <= 60 * 60 * 24 * 7
}

async fn get_token(user: &UserData) -> String {
    let token = user.get_credentials().await;

    match token {
        None => { new_token(user).await }
        Some(t) => {
            if validate_token_time(&t) {
                t
            } else {
                new_token(user).await
            }
        }
    }
}

pub async fn get_user_id(token: &str) -> Option<UserData> {
    if !validate_token_time(token) {
        return None;
    }

    UserData::get_by_credential(token).await
}

async fn delete_by_user(user: &UserData) {
    user.update_credentials(None).await.unwrap();
}

pub async fn delete_by_token(token: &str) {
    let user = UserData::get_by_credential(token).await.unwrap();
    delete_by_user(&user).await;
}

pub struct TokenInfo(pub UserData);
pub struct AdminTokenInfo(pub UserData);

impl<S> FromRequestParts<S> for TokenInfo
where
    S: Send + Sync,
{
    type Rejection = ErrorMessage;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;
        let token = headers.get("token")
            .ok_or(ErrorMessage::InvalidToken)?
            .to_str()
            .map_err(|_| ErrorMessage::InvalidToken)?;

        let user = get_user_id(token).await
            .ok_or(ErrorMessage::InvalidToken)?;

        Ok(TokenInfo(user))
    }
}

impl<S> FromRequestParts<S> for AdminTokenInfo
where
    S: Send + Sync,
{
    type Rejection = ErrorMessage;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;
        let token = headers.get("token")
            .ok_or(ErrorMessage::InvalidToken)?
            .to_str()
            .map_err(|_| ErrorMessage::InvalidToken)?;

        let user = get_user_id(token).await
            .ok_or(ErrorMessage::InvalidToken)?;

        if !user.admin {
            return Err(ErrorMessage::PermissionDenied);
        }

        Ok(AdminTokenInfo(user))
    }
}

impl UserData {
    pub async fn get_token(&self) -> String {
        get_token(self).await
    }

    pub async fn remove_token(&self) {
        delete_by_user(self).await;
    }
}
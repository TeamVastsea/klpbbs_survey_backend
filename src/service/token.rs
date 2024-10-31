use crate::controller::error::ErrorMessage;
use crate::dao::model::user_data::UserData;
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use chrono::Utc;
use rand::Rng;

async fn get_token(user: &UserData) -> String {
    let token = user.get_credentials().await;

    match token {
        None => {
            let time = Utc::now().timestamp();
            let new_token: String = rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(16)
                .map(char::from)
                .collect();
            let new_token = format!("{}-{}", time, new_token);
            user.update_credentials(Some(&new_token)).await.unwrap();

            new_token
        }
        Some(t) => { t }
    }
}

pub async fn get_user_id(token: &str) -> Option<UserData> {
    let time = token.split('-').next()?.parse::<i64>().ok()?;
    if Utc::now().timestamp() - time > 60 * 60 * 24 * 7 {
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

#[async_trait]
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

#[async_trait]
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
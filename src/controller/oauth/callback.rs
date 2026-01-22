use crate::controller::error::ErrorMessage;
use crate::dao::model::user_data::UserData;
use crate::OAUTH_CONFIG;
use axum::extract::Query;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::debug;

pub async fn oauth_callback(Query(query): Query<OauthCallbackQuery>) -> Result<String, ErrorMessage> {
    let data = get_oauth_login(query.token).await?;

    debug!("User data: {:?}", data);

    Ok(data.get_token().await)
}

async fn get_oauth_login(token: String) -> Result<UserData, ErrorMessage> {
    let res = Client::new()
        .get("https://klpbbs.com/plugin.php")
        .query(&OAuthLoginQuery::new(token))
        .send()
        .await
        .map_err(|e| {
            debug!("Failed to send request: {:?}", e);
            ErrorMessage::InvalidToken
        })?
        .text()
        .await
        .map_err(|e| {
            debug!("Failed to send request: {:?}", e);
            ErrorMessage::InvalidToken
        })?;

    debug!("Oauth replied: {:?}", res);

    let user: User = serde_json::from_str(&res).map_err(|_| ErrorMessage::InvalidToken)?;
    Ok(user.data.to_user_data().await)
}

#[derive(Deserialize, Debug)]
pub struct OauthCallbackQuery {
    pub token: String,
}

#[derive(Serialize, Debug)]
pub struct OAuthLoginQuery {
    id: &'static str,
    appid: &'static str,
    appkey: &'static str,
    token: String,
}

impl OAuthLoginQuery {
    pub fn new(token: String) -> Self {
        Self {
            id: "klpbbs_api:get_user_info",
            appid: &OAUTH_CONFIG.app_id,
            appkey: &OAUTH_CONFIG.app_key,
            token,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CallbackUserData {
    pub uid: String,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    code: i64,
    msg: String,
    time: i64,
    data: CallbackUserData,
}

impl CallbackUserData {
    async fn to_user_data(&self) -> UserData {
        let user = UserData::find_by_id(&self.uid).await;
        match user {
            Some(user) => user,
            None => {
                let user = UserData {
                    uid: self.uid.clone(),
                    username: self.username.clone(),
                    admin: false,
                };
                user.save().await.unwrap();
                user
            }
        }
    }
}
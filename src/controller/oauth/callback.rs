use crate::service::token::activate_token;
use crate::OAUTH_CONFIG;
use axum::extract::Query;
use futures::TryFutureExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub async fn oauth_callback(Query(query): Query<OauthCallbackQuery>) -> Result<(), String> {
    let data = get_oauth_login(query.token).await?;
    activate_token(&query.state, data).await;
    
    Ok(())
}

async fn get_oauth_login(token: String) -> Result<UserData, String> {
    let res = Client::new()
        .get("https://klpbbs.com/plugin.php")
        .query(&OAuthLoginQuery::new(token))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;
    
    let user: User = serde_json::from_str(&res).map_err(|_| res)?;
    Ok(user.data)
}

#[derive(Deserialize, Debug)]
pub struct OauthCallbackQuery {
    pub token: String,
    pub state: String,
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
pub struct UserData {
    pub uid: String,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    code: i64,
    msg: String,
    time: i64,
    data: UserData,
}
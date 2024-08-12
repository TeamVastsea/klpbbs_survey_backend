use axum::extract::Query;
use futures::TryFutureExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::debug;
use crate::OAUTH_CONFIG;
use crate::service::token::activate_token;

pub async fn oauth_callback(Query(query): Query<OauthCallbackQuery>) {
    let data = get_oauth_login(query.token).await.unwrap();
    activate_token(&query.state, data).await;
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
    
    let user: User = serde_json::from_str(&res).map_err(|e| e.to_string())?;
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
    uid: String,
    username: String,
    #[serde(skip_serializing)]
    groupid: String,
    #[serde(skip_serializing)]
    regdate: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    code: i64,
    msg: String,
    time: i64,
    data: UserData,
}
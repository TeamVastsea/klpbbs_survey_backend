use axum::extract::Query;
use futures::TryFutureExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::debug;
use crate::OAUTH_CONFIG;
use crate::service::token::activate_token;

pub async fn oauth_callback(Query(query): Query<OauthCallbackQuery>) {
    debug!("oauth_callback: {:?}", query);
    let (user, group) = get_oauth_login(query.token).await.unwrap();
    activate_token(&query.state, user).await;
    debug!("user: {}, group: {}, token: {}", user, group, query.state);
}

async fn get_oauth_login(token: String) -> Result< (i64, String), String> {
    let res = Client::new()
        .get("https://klpbbs.com/plugin.php")
        .query(&OAuthLoginQuery::new(token))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;
    
    println!("{:?}", res);
    Ok((1, "111".to_string()))
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
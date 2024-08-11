use std::time::Duration;
use lazy_static::lazy_static;
use moka::future::Cache;
use rand::Rng;

lazy_static! {
    static ref TOKEN_CACHE: Cache<String, Option<i64>> = Cache::builder()
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

pub async fn activate_token(token: &str, user_id: i64) {
    TOKEN_CACHE.insert(token.to_string(), Some(user_id)).await;
}

pub async fn get_user_id(token: &str) -> Option<i64> {
    TOKEN_CACHE.get(token).await.unwrap_or(None)
}
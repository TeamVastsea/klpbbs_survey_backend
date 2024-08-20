use crate::controller::error::ErrorMessage;
use crate::model::generated::admin;
use crate::model::generated::prelude::Admin;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use lazy_static::lazy_static;
use migration::async_trait::async_trait;
use moka::future::Cache;
use sea_orm::ColumnTrait;
use sea_orm::{EntityTrait, QueryFilter};
use std::time::Duration;

lazy_static! {
    static ref ADMIN_TOKEN_CACHE: Cache<String, admin::Model> = Cache::builder()
        .time_to_idle(Duration::from_secs(60 * 60 * 24 * 7)) //if the key is not accessed for 7 days, it will be removed
        .build();
}

pub async fn get_admin_by_id(id: i32) -> Option<admin::Model> {
    Admin::find()
        .filter(admin::Column::Id.eq(id))
        .one(&*crate::DATABASE).await.unwrap()
}

pub async fn register_admin_token(token: &str, user: i32) {
    let admin = get_admin_by_id(user).await.unwrap();
    ADMIN_TOKEN_CACHE.insert(token.to_string(), admin).await;
}

pub async fn get_admin_by_token(token: &str) -> Option<admin::Model> {
    ADMIN_TOKEN_CACHE.get(token).await
}

pub struct AdminTokenInfo(pub admin::Model);

#[async_trait]
impl<S> FromRequestParts<S> for AdminTokenInfo
where S: Send + Sync {
    type Rejection = ErrorMessage;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        ADMIN_TOKEN_CACHE.insert("222".to_string(), admin::Model {
            id: 222,
            username: "222".to_string(),
            disabled: false,
        }).await;
        
        let headers = &parts.headers;
        let token = headers.get("token")
            .ok_or(ErrorMessage::InvalidToken)?
            .to_str()
            .map_err(|_| ErrorMessage::InvalidToken)?;
        let user = get_admin_by_token(token).await
            .ok_or(ErrorMessage::TokenNotActivated)?;
        
        if user.disabled { 
            return Err(ErrorMessage::TokenNotActivated);
        }

        Ok(AdminTokenInfo(user))
    }
}
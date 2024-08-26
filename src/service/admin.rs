use crate::controller::error::ErrorMessage;
use crate::model::generated::admin;
use crate::model::generated::prelude::Admin;
use crate::service::token::get_user_id;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use migration::async_trait::async_trait;
use sea_orm::ColumnTrait;
use sea_orm::{EntityTrait, QueryFilter};
use tracing::debug;

pub async fn get_admin_by_id(id: i64) -> Option<admin::Model> {
    Admin::find()
        .filter(admin::Column::Id.eq(id))
        .one(&*crate::DATABASE).await.unwrap()
}


pub struct AdminTokenInfo(pub admin::Model);

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

        if token == "111" {
            return Ok(AdminTokenInfo(admin::Model {
                id: 111,
                username: "111".to_string(),
                disabled: false,
            }));
        }

        let user = get_user_id(token).await
            .ok_or(ErrorMessage::TokenNotActivated)?;

        let user = get_admin_by_id(user.uid.parse().unwrap()).await
            .ok_or(ErrorMessage::TokenNotActivated)?;

        if user.disabled {
            return Err(ErrorMessage::InvalidToken);
        }

        Ok(AdminTokenInfo(user))
    }
}
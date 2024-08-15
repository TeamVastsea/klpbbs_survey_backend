use axum::extract::Query;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::controller::error::ErrorMessage;
use crate::DATABASE;
use crate::model::generated::page;
use crate::model::generated::prelude::Page;
use crate::service::token::TokenInfo;

pub async fn get_page(Query(query): Query<GetPageQuery>, TokenInfo(user): TokenInfo) -> Result<String, ErrorMessage> {
    info!("User {} is trying to get page {}", user.uid, query.page);
    
    let Some(page) = Page::find().filter(page::Column::Id.eq(query.page)).one(&*DATABASE).await.unwrap()
        else { 
            return Err(ErrorMessage::NotFound);
        };
    
    Ok(serde_json::to_string(&page).unwrap())
}

#[derive(Deserialize)]
pub struct GetPageQuery {
    page: String,
}
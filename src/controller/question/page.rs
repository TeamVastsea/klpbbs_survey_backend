use crate::controller::error::ErrorMessage;
use crate::model::generated::page;
use crate::service::admin::AdminTokenInfo;
use crate::service::questions::get_page_by_id;
use crate::service::token::TokenInfo;
use axum::extract::Query;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, NotSet};
use serde::Deserialize;
use tracing::info;
use uuid::Uuid;

pub async fn get_page(Query(query): Query<GetPageQuery>, TokenInfo(user): TokenInfo) -> Result<String, ErrorMessage> {
    info!("User {} is trying to get page {}", user.uid, query.page);
    
    let Some(page) = get_page_by_id(&query.page).await
        else { 
            return Err(ErrorMessage::NotFound);
        };
    
    Ok(serde_json::to_string(&page).unwrap())
}

pub async fn new_page(Query(query): Query<CreatePageRequest>, AdminTokenInfo(admin): AdminTokenInfo) -> String {
    info!("Admin {} create new page", admin.id);
    let id = Uuid::new_v4().to_string();
    
    let page = page::ActiveModel {
        id: Set(id),
        title: Set(query.title),
        content: Set(Vec::new()),
        next: NotSet,
    };
    
    let page = page.insert(&*crate::DATABASE).await.unwrap();
    
    page.id
}

#[derive(Deserialize)]
pub struct GetPageQuery {
    page: String,
}

#[derive(Deserialize)]
pub struct CreatePageRequest {
    title: String,
}
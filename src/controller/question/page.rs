use crate::controller::error::ErrorMessage;
use crate::model::generated::page;
use crate::service::admin::AdminTokenInfo;
use crate::service::questions::{get_page_by_id, save_page};
use crate::service::token::TokenInfo;
use axum::extract::Query;
use axum::Json;
use serde::Deserialize;
use tracing::info;

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

    save_page(query.title, Vec::new(), None, None, None).await
}

pub async fn modify_page(AdminTokenInfo(admin): AdminTokenInfo, Json(body): Json<page::Model>) -> Result<String, ErrorMessage> {
    info!("Admin {} modify page {}", admin.id, body.id);

    let result = save_page(body.title, body.content, body.next, body.previous, Some(body.id.clone())).await;

    Ok(result)
}

#[derive(Deserialize)]
pub struct GetPageQuery {
    page: String,
}

#[derive(Deserialize)]
pub struct CreatePageRequest {
    title: String,
}
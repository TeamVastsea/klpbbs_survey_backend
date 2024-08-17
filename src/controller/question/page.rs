use crate::controller::error::ErrorMessage;
use crate::service::questions::get_page_by_id;
use crate::service::token::TokenInfo;
use axum::extract::Query;
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

#[derive(Deserialize)]
pub struct GetPageQuery {
    page: String,
}
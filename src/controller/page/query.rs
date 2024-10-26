use crate::controller::error::ErrorMessage;
use crate::dao::entity::page;
use crate::service::token::TokenInfo;
use axum::extract::{Path, Query};
use serde::{Deserialize, Serialize};

pub async fn get_page(Path(id): Path<String>, TokenInfo(user): TokenInfo) -> Result<String, ErrorMessage> {
    let page = page::Model::find_by_id(&id).await?;
    
    if !user.admin { 
        let (allow_submit, ..) = page.check_access().await?;
        
        if !allow_submit {
            return Err(ErrorMessage::PermissionDenied);
        }
    }
    
    Ok(serde_json::to_string(&page).unwrap())
}

pub async fn get_page_by_index(Query(query): Query<IndexPageQuery>, TokenInfo(user): TokenInfo) -> Result<String, ErrorMessage> {
    let pages = page::Model::get_by_survey_and_index(&query.survey, query.index).await?;
    
    if !user.admin {
        let (allow_submit, ..) = pages.0.check_access().await?;

        if !allow_submit {
            return Err(ErrorMessage::PermissionDenied);
        }
    }
    
    let response = IndexPageResponse {
        data: pages.0,
        total: pages.1
    };
    
    Ok(serde_json::to_string(&response).unwrap())
}

#[derive(Deserialize)]
pub struct GetPageQuery {
    page: String,
}

#[derive(Deserialize)]
pub struct IndexPageQuery {
    survey: String,
    index: u64
}

#[derive(Serialize)]
pub struct IndexPageResponse {
    data: page::Model,
    total: u64,
}
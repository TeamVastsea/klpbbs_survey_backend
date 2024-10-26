use axum::extract::{Query, Json};
use sea_orm::{ActiveModelTrait, IntoActiveModel};
use serde::Deserialize;
use tracing::info;
use crate::controller::error::ErrorMessage;
use crate::dao::entity::page;
use crate::service::token::AdminTokenInfo;

pub async fn new_page(AdminTokenInfo(admin): AdminTokenInfo, Json(query): Json<CreatePageRequest>) -> String {
    info!("Admin {} create new page", admin.uid);
    
    let result = page::Model::new_page(query.title, query.survey).await;
    
    serde_json::to_string(&result).unwrap()
}

pub async fn modify_page(AdminTokenInfo(admin): AdminTokenInfo, Json(body): Json<page::Model>) -> Result<String, ErrorMessage> {
    info!("Admin {} modify page {}", admin.uid, body.id);

    let page = body.into_active_model().reset_all();
    
    page.update(&*crate::DATABASE).await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))
        .map(|result| result.id.to_string())
}

#[derive(Deserialize)]
pub struct CreatePageRequest {
    title: String,
    survey: i32,
}
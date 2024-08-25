use crate::controller::error::ErrorMessage;
use crate::model::generated::prelude::Admin;
use crate::service::admin::AdminTokenInfo;
use axum::extract::Query;
use sea_orm::ColumnTrait;
use sea_orm::{EntityTrait, QueryFilter};
use serde::Deserialize;
use tracing::info;

pub async fn get_admin_info(Query(query): Query<AdminInfoRequest>, AdminTokenInfo(admin): AdminTokenInfo) -> Result<String, ErrorMessage> {
    info!("Admin {} is getting info of admin {}", admin.id, query.id);

    let admin = Admin::find()
        .filter(crate::model::generated::admin::Column::Id.eq(query.id))
        .one(&*crate::DATABASE).await.unwrap()
        .ok_or(ErrorMessage::NotFound)?;

    Ok(serde_json::to_string(&admin).unwrap())
}

#[derive(Deserialize)]
pub struct AdminInfoRequest {
    pub id: i64,
}
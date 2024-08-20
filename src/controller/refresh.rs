use axum::extract::Query;
use serde::Deserialize;
use tracing::info;
use crate::service::admin::AdminTokenInfo;
use crate::service::questions::CacheType;

pub async fn refresh_cache(Query(query): Query<RefreshType>, AdminTokenInfo(admin): AdminTokenInfo) {
    info!("Admin {} is refreshing {:?} cache", admin.id, query.r#type);
    
    crate::service::questions::refresh_cache(query.r#type);
}

#[derive(Deserialize)]
pub struct RefreshType {
    pub r#type: CacheType,
}
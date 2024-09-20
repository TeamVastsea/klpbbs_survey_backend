use crate::controller::error::ErrorMessage;
use crate::service::admin::AdminTokenInfo;
use axum::extract::Query;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;
use uuid::Uuid;
use crate::model::judge::{get_judge_result};

pub async fn auto_judge(Query(query): Query<JudgeRequest>, AdminTokenInfo(admin): AdminTokenInfo) -> Result<String, ErrorMessage> {
    info!("Admin {} is judging answer {}", admin.id, query.answer);

    let result = get_judge_result(query.answer, admin.id).await?;

    Ok(serde_json::to_string(&result).unwrap())
}

#[derive(Deserialize)]
pub struct JudgeRequest {
    pub answer: i32,
}

#[derive(Serialize)]
pub struct JudgeResponse {
    pub full: i32,
    pub user: i32,
    pub scores: HashMap<Uuid, i32>,
    pub completed: bool,
}
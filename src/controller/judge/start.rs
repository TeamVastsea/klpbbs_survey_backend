use crate::controller::error::ErrorMessage;
use crate::model::generated::prelude::{Answer, Page, Survey};
use crate::model::question::Question;
use crate::service::admin::AdminTokenInfo;
use crate::service::judge::judge_subjectives;
use crate::service::questions::get_question_by_id;
use crate::DATABASE;
use axum::extract::Query;
use sea_orm::ColumnTrait;
use sea_orm::{EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;
use uuid::Uuid;
use crate::model::judge::{get_judge_result};

pub async fn auto_judge(Query(query): Query<JudgeRequest>, AdminTokenInfo(admin): AdminTokenInfo) -> Result<String, ErrorMessage> {
    info!("Admin {} is judging answer {}", admin.id, query.answer);
    
    let (scores, user, full, completed) = get_judge_result(query.answer, admin.id).await?;

    let response = JudgeResponse {
        full,
        user,
        scores,
        completed
    };

    Ok(serde_json::to_string(&response).unwrap())
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
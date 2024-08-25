use crate::controller::error::ErrorMessage;
use crate::model::generated::prelude::{Answer, Score};
use crate::model::generated::{answer, score};
use crate::service::admin::AdminTokenInfo;
use axum::extract::Query;
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};
use serde::Deserialize;
use tracing::info;

pub async fn confirm_judge(Query(query): Query<ConfirmJudgeRequest>, AdminTokenInfo(admin): AdminTokenInfo) -> Result<(), ErrorMessage> {
    let score = Score::find()
        .filter(score::Column::Id.eq(query.answer))
        .one(&*crate::DATABASE).await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?
        .ok_or(ErrorMessage::NotFound)?;

    let mut answer = Answer::find()
        .filter(answer::Column::Id.eq(query.answer))
        .one(&*crate::DATABASE).await.unwrap()
        // .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?
        .ok_or(ErrorMessage::NotFound)?
        .into_active_model();

    info!("Admin {} is confirming judge {}", admin.id, query.answer);

    answer.score = Set(Some(score.user_score));
    answer.save(&*crate::DATABASE).await.unwrap();

    let mut score = score.into_active_model();
    score.completed = Set(true);
    score.judge_time = Set(Utc::now().naive_utc());
    score.judge = Set(admin.id);
    score.save(&*crate::DATABASE).await.unwrap();

    Ok(())
}

#[derive(Deserialize)]
pub struct ConfirmJudgeRequest {
    pub answer: i32,
}
use sea_orm::{FromQueryResult, QueryFilter, QuerySelect, SelectColumns};
use sea_orm::ColumnTrait;
use axum::extract::Query;
use sea_orm::EntityTrait;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use crate::controller::error::ErrorMessage;
use crate::dao::entity::prelude::Score;
use crate::dao::entity::score;
use crate::DATABASE;
use crate::service::token::TokenInfo;

pub async fn get_by_user(Query(query): Query<GetByUserRequest>, TokenInfo(user): TokenInfo) -> Result<String, ErrorMessage> {
    let scores = Score::find()
        .filter(score::Column::User.eq(&user.uid))
        .filter(score::Column::Survey.eq(query.survey))
        .filter(score::Column::Completed.eq(false))
        .select_only()
        .select_column(score::Column::Id)
        .select_column(score::Column::Answer)
        .select_column(score::Column::UpdateTime)
        .into_model::<ScorePrompt>()
        .all(&*DATABASE).await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;

    Ok(serde_json::to_string(&scores).unwrap())
}

#[derive(serde::Deserialize)]
pub struct GetByUserRequest {
    pub survey: i32,
}

#[derive(Serialize, FromQueryResult)]
struct ScorePrompt {
    pub id: i32,
    pub answer: String,
    pub update_time: DateTime,
}

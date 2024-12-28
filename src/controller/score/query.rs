use crate::controller::error::ErrorMessage;
use crate::dao::entity::prelude::Score;
use crate::dao::entity::score;
use crate::dao::model::PagedData;
use crate::service::token::{AdminTokenInfo, TokenInfo};
use crate::DATABASE;
use axum::extract::{Path, Query};
use log::info;
use sea_orm::prelude::DateTime;
use sea_orm::{EntityTrait, QueryOrder};
use sea_orm::{ColumnTrait, PaginatorTrait};
use sea_orm::{FromQueryResult, QueryFilter, QuerySelect, SelectColumns};
use serde::{Deserialize, Serialize};
use std::cmp::min;
use migration::Order;

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

pub async fn get_by_id(Path(id): Path<i32>, AdminTokenInfo(user): AdminTokenInfo) -> Result<String, ErrorMessage> {
    info!("Admin {} get score {}", user.uid, id);
    
    let score = Score::find_by_id(id).one(&*DATABASE).await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?
        .ok_or(ErrorMessage::NotFound)?;

    Ok(serde_json::to_string(&score).unwrap())
}

pub async fn search_answer(Query(request): Query<SearchAnswerQuery>, AdminTokenInfo(user): AdminTokenInfo) -> Result<String, ErrorMessage> {
    info!("Admin {} search answer", user.uid);

    let mut answers = Score::find();

    if let Some(survey) = request.survey {
        answers = answers.filter(score::Column::Survey.eq(survey));
    }
    if let Some(user) = request.user {
        answers = answers.filter(score::Column::User.eq(user));
    }
    if let Some(true) = request.only_unfinished {
        answers = answers.filter(score::Column::Judge.is_null());
    }

    let page = min(request.size.unwrap_or(10), 20);

    let answers = answers
        .select_only()
        .column(score::Column::Id)
        .column(score::Column::Survey)
        .column(score::Column::User)
        .column(score::Column::UpdateTime)
        .column_as(score::Column::Judge.is_not_null(), "completed")
        .order_by(score::Column::UpdateTime, Order::Desc)
        .into_model::<ScoreInfo>()
        .paginate(&*DATABASE, page);

    let result = PagedData {
        data: answers.fetch_page(request.page).await.map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?,
        total: answers.num_pages().await.map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?,
    };

    Ok(serde_json::to_string(&result).unwrap())
}

#[derive(Deserialize)]
pub struct GetByUserRequest {
    pub survey: i32,
}

#[derive(Serialize, FromQueryResult)]
struct ScorePrompt {
    pub id: i32,
    pub answer: String,
    pub update_time: DateTime,
}

#[derive(Deserialize)]
pub struct SearchAnswerQuery {
    page: u64,
    size: Option<u64>,
    survey: Option<i32>,
    user: Option<i64>,
    only_unfinished: Option<bool>,
}

#[derive(Serialize, FromQueryResult)]
pub struct ScoreInfo {
    pub id: i32,
    pub survey: i32,
    pub user: String,
    pub update_time: DateTime,
    pub completed: bool,
}
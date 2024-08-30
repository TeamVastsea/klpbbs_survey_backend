use crate::controller::error::ErrorMessage;
use crate::model::generated::prelude::Answer;
use crate::model::generated::{answer, score};
use crate::service::admin::AdminTokenInfo;
use crate::DATABASE;
use axum::extract::Query;
use sea_orm::prelude::DateTime;
use sea_orm::{ColumnTrait, FromQueryResult, QuerySelect};
use sea_orm::{EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tracing::info;

pub async fn search_answer(AdminTokenInfo(admin): AdminTokenInfo, Query(request): Query<SearchAnswerQuery>) -> Result<String, ErrorMessage> {
    info!("Admin {} search answer", admin.id);

    let mut answers = Answer::find();

    if let Some(survey) = request.survey {
        answers = answers.filter(answer::Column::Survey.eq(survey));
    }
    if let Some(user) = request.user {
        answers = answers.filter(answer::Column::User.eq(user));
    }
    if let Some(true) = request.only_unfinished {
        answers = answers.filter(score::Column::Completed.eq(false));
    }

    let answers = answers
        .columns(vec![
            answer::Column::Id,
            answer::Column::Survey,
            answer::Column::User,
            answer::Column::Score,
            answer::Column::CreateTime,
        ])
        .column_as(answer::Column::Completed, "done")
        .column(score::Column::Completed)
        .filter(answer::Column::Completed.eq(true))
        .inner_join(score::Entity)
        .into_model::<AnswerQueryResult>();

    let paginator = answers.paginate(&*DATABASE, request.size.unwrap_or(10));
    let answers = paginator.fetch_page(request.page).await.unwrap();

    let answers = SearchAnswerResponse {
        records: answers,
        total: paginator.num_pages().await.unwrap(),
    };

    Ok(serde_json::to_string(&answers).unwrap())
}

#[derive(Deserialize)]
pub struct SearchAnswerQuery {
    page: u64,
    size: Option<u64>,
    survey: Option<i32>,
    user: Option<i64>,
    only_unfinished: Option<bool>,
}

#[derive(Serialize)]
pub struct SearchAnswerResponse {
    records: Vec<AnswerQueryResult>,
    total: u64,
}

#[derive(Serialize, FromQueryResult)]
pub struct AnswerQueryResult {
    pub id: i32,
    pub survey: i32,
    pub user: i64,
    pub score: Option<i32>,
    pub create_time: DateTime,
    pub completed: bool,
}
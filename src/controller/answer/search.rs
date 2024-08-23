use axum::extract::Query;
use sea_orm::{EntityTrait, PaginatorTrait, QueryFilter};
use tracing::info;
use crate::controller::error::ErrorMessage;
use crate::DATABASE;
use crate::model::generated::prelude::Answer;
use crate::service::admin::AdminTokenInfo;
use sea_orm::ColumnTrait;
use serde::Deserialize;

pub async fn search_answer(AdminTokenInfo(admin): AdminTokenInfo, Query(request): Query<SearchAnswerQuery>) -> Result<String, ErrorMessage> {
    info!("Admin {} search answer", admin.id);

    let mut answers = Answer::find();

    if let Some(survey) = request.survey {
        answers = answers.filter(crate::model::generated::answer::Column::Survey.eq(survey));
    }
    if let Some(user) = request.user {
        answers = answers.filter(crate::model::generated::answer::Column::User.eq(user));
    }
    
    let answers = answers.paginate(&*DATABASE, request.size.unwrap_or(10));
    let answers = answers.fetch_page(request.page).await.unwrap();

    Ok(serde_json::to_string(&answers).unwrap())
}

#[derive(Deserialize)]
pub struct SearchAnswerQuery {
    pub page: u64,
    pub size: Option<u64>,
    pub survey: Option<i32>,
    pub user: Option<i64>,
}
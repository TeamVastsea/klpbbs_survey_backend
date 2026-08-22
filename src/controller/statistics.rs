use crate::DATABASE;
use crate::controller::error::ErrorMessage;
use crate::dao::entity::prelude::{Score, Survey, User};
use crate::dao::entity::{score, survey};
use crate::service::token::AdminTokenInfo;
use axum::Json;
use chrono::{Duration, Local, Utc};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use serde::Serialize;

#[derive(Serialize)]
pub struct Statistics {
    surveys: u64,
    available_surveys: u64,
    submissions: u64,
    recent_submissions: u64,
    users: u64,
}

pub async fn get_statistics(
    AdminTokenInfo(_admin): AdminTokenInfo,
) -> Result<Json<Statistics>, ErrorMessage> {
    let current_time = Local::now().naive_local();
    let one_week_ago = Utc::now().naive_utc() - Duration::weeks(1);

    let surveys = Survey::find()
        .count(&*DATABASE)
        .await
        .map_err(database_error)?;
    let available_surveys = Survey::find()
        .filter(survey::Column::AllowSubmit.eq(true))
        .filter(survey::Column::AllowView.eq(true))
        .filter(survey::Column::StartDate.lte(current_time))
        .filter(survey::Column::EndDate.gte(current_time))
        .count(&*DATABASE)
        .await
        .map_err(database_error)?;
    let submissions = Score::find()
        .filter(score::Column::Completed.eq(true))
        .count(&*DATABASE)
        .await
        .map_err(database_error)?;
    let recent_submissions = Score::find()
        .filter(score::Column::Completed.eq(true))
        .filter(score::Column::UpdateTime.gte(one_week_ago))
        .count(&*DATABASE)
        .await
        .map_err(database_error)?;
    let users = User::find()
        .count(&*DATABASE)
        .await
        .map_err(database_error)?;

    Ok(Json(Statistics {
        surveys,
        available_surveys,
        submissions,
        recent_submissions,
        users,
    }))
}

fn database_error(error: sea_orm::DbErr) -> ErrorMessage {
    ErrorMessage::DatabaseError(error.to_string())
}

use crate::controller::error::ErrorMessage;
use crate::dao::entity::prelude::{Score, Survey};
use crate::dao::entity::{score, survey};
use crate::service::score::combine_answer;
use crate::service::token::{AdminTokenInfo, TokenInfo};
use crate::DATABASE;
use axum::extract::{Path, Query};
use axum::Json;
use log::info;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter, Select, TryIntoModel};
use serde::Deserialize;
use serde_json::Value;

fn find_open_survey(id: i32, now: chrono::NaiveDateTime) -> Select<survey::Entity> {
    Survey::find_by_id(id)
        .filter(survey::Column::AllowSubmit.eq(true))
        .filter(survey::Column::StartDate.lte(now))
        .filter(survey::Column::EndDate.gte(now))
}

fn find_updatable_score(id: i32, user: &str, survey: i32) -> Select<score::Entity> {
    Score::find_by_id(id)
        .filter(score::Column::User.eq(user))
        .filter(score::Column::Survey.eq(survey))
        .filter(score::Column::Completed.eq(false))
        .filter(score::Column::Judge.is_null())
}

pub async fn submit(TokenInfo(user): TokenInfo, Json(request): Json<SubmitBody>) -> Result<String, ErrorMessage> {
    if !request.content.is_object() { 
        return Err(ErrorMessage::InvalidParams("content".to_string()));
    }

    let survey = find_open_survey(request.survey, chrono::Local::now().naive_local())
        .one(&*DATABASE).await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?
        .ok_or(ErrorMessage::NotFound)?;

    let score = match request.id {
        None => {
            let count = Score::find()
                .filter(score::Column::User.eq(&user.uid))
                .filter(score::Column::Survey.eq(request.survey))
                .count(&*DATABASE).await.map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;
            if count > 0 && !survey.allow_re_submit {
                return Err(ErrorMessage::TooManySubmit);
            }

            score::ActiveModel::new(&user.uid, request.content, request.survey)
        }
        Some(id) => {
            let model = find_updatable_score(id, &user.uid, request.survey)
                .one(&*DATABASE).await
                .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?
                .ok_or(ErrorMessage::NotFound)?;

            let answer_combined = combine_answer(serde_json::from_str(&model.answer).unwrap(), request.content);
            let mut model = model.into_active_model();
            model.answer = Set(serde_json::to_string(&answer_combined).unwrap());
            model.update_time = Set(chrono::Utc::now().naive_local());

            model
        }
    };

    let result = score.save(&*DATABASE).await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?
        .try_into_model()
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;

    info!("{:?}", result);

    Ok(result.id.to_string())
}

pub async fn finish(TokenInfo(user): TokenInfo, Query(query): Query<FinishQuery>) -> Result<(), ErrorMessage> {
    let score = Score::find_by_id(query.id)
        .filter(score::Column::User.eq(&user.uid))
        .filter(score::Column::Completed.eq(false))
        .filter(score::Column::Judge.is_null())
        .one(&*DATABASE).await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?
        .ok_or(ErrorMessage::NotFound)?;

    find_open_survey(score.survey, chrono::Local::now().naive_local())
        .one(&*DATABASE).await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?
        .ok_or(ErrorMessage::NotFound)?;

    let mut score = score.into_active_model();

    score.completed = Set(true);
    score.update_time = Set(chrono::Utc::now().naive_local());

    let score = score.update(&*DATABASE).await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;
    score.judge_answer().await;

    Ok(())
}

pub async fn rejudge(Path(id): Path<i32>, AdminTokenInfo(admin): AdminTokenInfo) -> Result<String, ErrorMessage> {
    let score = Score::find_by_id(id)
        .filter(score::Column::Completed.eq(true))
        .filter(score::Column::Judge.is_null())
        .one(&*DATABASE).await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?
        .ok_or(ErrorMessage::NotFound)?;

    if score.judge.is_some() {
        return Err(ErrorMessage::TooManySubmit);
    }

    info!("Admin {} rejudge score {}", admin.uid, id);

    Ok(serde_json::to_string(&score.judge_answer().await).unwrap())
}

pub async fn confirm(Path(id): Path<i32>, AdminTokenInfo(admin): AdminTokenInfo) -> Result<(), ErrorMessage> {
    info!("Admin {} confirm score {}", admin.uid, id);
    let mut score = Score::find_by_id(id)
        .filter(score::Column::Completed.eq(true))
        .filter(score::Column::Judge.is_null())
        .one(&*DATABASE).await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?
        .ok_or(ErrorMessage::NotFound)?.into_active_model();

    score.judge = Set(Some(admin.uid));
    score.update_time = Set(chrono::Utc::now().naive_local());

    score.update(&*DATABASE).await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;

    Ok(())
}

#[derive(Deserialize)]
pub struct SubmitBody {
    id: Option<i32>,
    content: Value,
    survey: i32,
}

#[derive(Deserialize)]
pub struct FinishQuery {
    id: i32,
}

#[cfg(test)]
mod tests {
    use super::{find_open_survey, find_updatable_score};
    use chrono::NaiveDate;
    use sea_orm::{DbBackend, QueryTrait};

    #[test]
    fn unfinished_score_lookup_is_scoped_to_owner() {
        let statement = find_updatable_score(42, "owner-1", 7).build(DbBackend::Postgres);
        let sql = statement.to_string();

        assert!(sql.contains(r#""score"."id" = 42"#));
        assert!(sql.contains(r#""score"."user" = 'owner-1'"#));
        assert!(sql.contains(r#""score"."survey" = 7"#));
        assert!(sql.contains(r#""score"."completed" = FALSE"#));
        assert!(sql.contains(r#""score"."judge" IS NULL"#));
    }

    #[test]
    fn survey_lookup_requires_an_open_submission_window() {
        let now = NaiveDate::from_ymd_opt(2026, 8, 21)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap();
        let statement = find_open_survey(7, now).build(DbBackend::Postgres);
        let sql = statement.to_string();

        assert!(sql.contains(r#""survey"."id" = 7"#));
        assert!(sql.contains(r#""survey"."allow_submit" = TRUE"#));
        assert!(sql.contains(r#""survey"."start_date" <= '2026-08-21 12:00:00'"#));
        assert!(sql.contains(r#""survey"."end_date" >= '2026-08-21 12:00:00'"#));
    }
}

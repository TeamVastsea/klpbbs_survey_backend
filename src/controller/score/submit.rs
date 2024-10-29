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
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, IntoActiveModel, PaginatorTrait, QueryFilter, QuerySelect, SelectColumns, TryIntoModel};
use serde::Deserialize;
use serde_json::Value;

pub async fn submit(TokenInfo(user): TokenInfo, Json(request): Json<SubmitBody>) -> Result<String, ErrorMessage> {
    #[derive(FromQueryResult)]
    struct SurveyAllowReSubmit {
        allow_re_submit: bool,
    }

    let score = match request.id {
        None => {
            let count = Score::find()
                .filter(score::Column::User.eq(&user.uid))
                .filter(score::Column::Survey.eq(request.survey))
                .count(&*DATABASE).await.map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;
            if count > 0 {
                let survey = Survey::find_by_id(request.survey)
                    .select_only()
                    .select_column(survey::Column::AllowReSubmit)
                    .into_model::<SurveyAllowReSubmit>()
                    .one(&*DATABASE).await
                    .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?
                    .ok_or(ErrorMessage::NotFound)?;
                if !survey.allow_re_submit {
                    return Err(ErrorMessage::TooManySubmit);
                }
            }

            score::ActiveModel::new(&user.uid, request.content, request.survey)
        }
        Some(id) => {
            let model = Score::find()
                .filter(score::Column::Id.eq(id))
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
    let mut score = Score::find_by_id(query.id)
        .filter(score::Column::User.eq(&user.uid))
        .one(&*DATABASE).await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?
        .ok_or(ErrorMessage::NotFound)?.into_active_model();

    score.completed = Set(true);
    score.update_time = Set(chrono::Utc::now().naive_local());

    let score = score.update(&*DATABASE).await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;
    score.judge_answer().await;

    Ok(())
}

pub async fn rejudge(Path(id): Path<i32>, AdminTokenInfo(admin): AdminTokenInfo) -> Result<String, ErrorMessage> {
    let score = Score::find_by_id(id)
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
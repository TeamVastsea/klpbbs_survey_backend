use crate::controller::error::ErrorMessage;
use crate::dao::entity::survey;
use crate::service::token::AdminTokenInfo;
use crate::DATABASE;
use ammonia::clean;
use axum::Json;
use sea_orm::prelude::DateTime;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, IntoActiveModel, NotSet};
use serde::Deserialize;
use tracing::info;

pub async fn modify_survey(AdminTokenInfo(admin): AdminTokenInfo, Json(request): Json<survey::Model>) -> Result<String, ErrorMessage> {
    info!("Admin {} modify survey {}", admin.uid, request.id);
    let survey = survey::Model {
        description: clean(&request.description),
        ..request
    };
    let survey = survey.into_active_model().reset_all();

    let result = survey.update(&*DATABASE).await.map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;
    Ok(result.id.to_string())
}

pub async fn create_survey(AdminTokenInfo(admin): AdminTokenInfo, Json(request): Json<CreateSurveyRequest>) -> Result<String, ErrorMessage> {
    info!("Admin {} create survey", admin.uid);
    let survey = survey::ActiveModel {
        id: NotSet,
        title: Set(request.title),
        badge: Set(request.badge),
        description: Set(clean(&request.description)),
        image: Set(request.image),
        start_date: Set(request.start_date),
        end_date: Set(request.end_date),
        allow_submit: Set(request.allow_submit),
        allow_view: Set(request.allow_view),
        allow_judge: Set(request.allow_judge),
        allow_re_submit: Set(request.allow_re_submit),
    };

    let survey = survey.insert(&*DATABASE).await.map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;

    Ok(survey.id.to_string())
}

#[derive(Deserialize)]
pub struct CreateSurveyRequest {
    pub title: String,
    pub badge: String,
    pub description: String,
    pub image: String,
    pub start_date: DateTime,
    pub end_date: DateTime,
    pub allow_submit: bool,
    pub allow_view: bool,
    pub allow_judge: bool,
    pub allow_re_submit: bool,
}
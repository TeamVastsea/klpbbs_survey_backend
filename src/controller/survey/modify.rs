use crate::controller::error::ErrorMessage;
use crate::dao::entity::{page, survey};
use crate::service::token::AdminTokenInfo;
use crate::DATABASE;
use crate::dao::deserialize_datetime_as_z;
use ammonia::clean;
use axum::Json;
use sea_orm::prelude::DateTime;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, IntoActiveModel, NotSet};
use serde::Deserialize;
use tracing::info;

pub async fn modify_survey(AdminTokenInfo(admin): AdminTokenInfo, Json(request): Json<survey::Model>) -> Result<String, ErrorMessage> {
    info!("Admin {} modify survey {}", admin.uid, request.id);
    let survey = normalize_survey(request);
    let survey = survey.into_active_model().reset_all();

    let result = survey.update(&*DATABASE).await.map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;
    Ok(result.id.to_string())
}

pub async fn create_survey(AdminTokenInfo(admin): AdminTokenInfo, Json(request): Json<CreateSurveyRequest>) -> Result<String, ErrorMessage> {
    info!("Admin {} create survey", admin.uid);
    let survey = survey::ActiveModel {
        id: NotSet,
        title: Set(request.title.clone()),
        badge: Set(request.badge),
        description: Set(clean(&request.description)),
        image: Set(request.image),
        start_date: Set(request.start_date),
        end_date: Set(request.end_date),
        allow_submit: Set(request.allow_submit),
        allow_view: Set(request.allow_view),
        allow_judge: Set(true),
        allow_re_submit: Set(request.allow_re_submit),
    };

    let survey = survey.insert(&*DATABASE).await.map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;
    
    page::Model::new_page(request.title, survey.id, 1).await;

    Ok(survey.id.to_string())
}

#[derive(Deserialize)]
pub struct CreateSurveyRequest {
    pub title: String,
    pub badge: String,
    pub description: String,
    pub image: String,
    #[serde(deserialize_with = "deserialize_datetime_as_z")]
    pub start_date: DateTime,
    #[serde(deserialize_with = "deserialize_datetime_as_z")]
    pub end_date: DateTime,
    pub allow_submit: bool,
    pub allow_view: bool,
    pub allow_re_submit: bool,
}

fn normalize_survey(request: survey::Model) -> survey::Model {
    survey::Model {
        description: clean(&request.description),
        allow_judge: true,
        ..request
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_survey;
    use crate::dao::entity::survey;
    use chrono::NaiveDate;

    #[test]
    fn survey_updates_always_enable_judging() {
        let timestamp = NaiveDate::from_ymd_opt(2026, 8, 22)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let survey = survey::Model {
            id: 1,
            title: "Survey".to_string(),
            badge: "".to_string(),
            description: "Description".to_string(),
            image: "".to_string(),
            start_date: timestamp,
            end_date: timestamp,
            allow_submit: true,
            allow_view: true,
            allow_judge: false,
            allow_re_submit: false,
        };

        assert!(normalize_survey(survey).allow_judge);
    }
}

use crate::controller::error::ErrorMessage;
use crate::model::generated::{answer, survey};
use crate::model::generated::prelude::{Answer, Survey};
use crate::service::token::TokenInfo;
use crate::DATABASE;
use axum::Json;
use sea_orm::{ActiveModelTrait, ColumnTrait, FromQueryResult, NotSet, QuerySelect, SelectColumns};
use sea_orm::{EntityTrait, IntoActiveModel, QueryFilter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sea_orm::ActiveValue::Set;
use serde_json::json;

pub async fn submit_answer(TokenInfo(user): TokenInfo, Json(request): Json<SubmitRequest>) -> Result<String, ErrorMessage> {
    if request.complete.unwrap_or(false) { 
        let survey = Survey::find()
            .filter(survey::Column::Id.eq(request.survey))
            .filter(survey::Column::AllowSubmit.eq(true))
            .filter(survey::Column::StartDate.lte(chrono::Utc::now()))
            .filter(survey::Column::EndDate.gte(chrono::Utc::now()))
            .select_only()
            .select_column(survey::Column::AllowReSubmit)
            .into_model::<SurveyControlQuery>()
            .one(&*DATABASE).await.unwrap()
            .ok_or(ErrorMessage::NotFound)?;
        
        println!("{:?}", survey);
        
        if !survey.allow_re_submit && Answer::find()
                .filter(answer::Column::User.eq(user.uid.parse::<i64>().unwrap()))
                .filter(answer::Column::Survey.eq(request.survey))
                .filter(answer::Column::Completed.eq(true))
                .select_only()
                .into_json()
                .one(&*DATABASE)
                .await.unwrap().is_some() {
            return Err(ErrorMessage::TooManySubmit);
        }
        
        return Ok("".to_string());
    }
    
    let mut new_answer = json!({});

    let mut answer = if let Some(old_answer_id) = request.id {
        let old = Answer::find()
            .filter(answer::Column::Id.eq(old_answer_id))
            .filter(answer::Column::User.eq(user.uid.parse::<i64>().unwrap()))
            .filter(answer::Column::Survey.eq(request.survey))
            .filter(answer::Column::Completed.eq(false))
            .one(&*DATABASE).await.unwrap()
            .ok_or(ErrorMessage::NotFound)?;
        new_answer = old.answers.clone();
        
        old.into_active_model()
    } else {
        answer::ActiveModel {
            id: NotSet,
            survey: Set(request.survey),
            user: Set(user.uid.parse::<i64>().unwrap()),
            judge: NotSet,
            answers: NotSet,
            score: NotSet,
            create_time: NotSet,
            judged_time: NotSet,
            completed: NotSet,
        }
    };
    
    for (key, value) in request.content {
        if !value.is_empty() { 
            new_answer[key] = value.into();
        }
    }
    
    answer.answers = Set(new_answer);
    answer.completed = Set(request.complete.unwrap_or(false));
    
    let ret = answer.save(&*DATABASE).await.unwrap();

    Ok(ret.id.unwrap().to_string())
}

#[derive(FromQueryResult, Serialize, Debug)]
pub struct SurveyControlQuery {
    pub allow_re_submit: bool,
}

#[derive(Deserialize)]
pub struct SubmitRequest {
    id: Option<i32>,
    complete: Option<bool>,
    survey: i32,
    content: HashMap<String, String>,
}
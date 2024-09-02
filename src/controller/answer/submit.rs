use crate::controller::error::ErrorMessage;
use crate::model::generated::prelude::{Answer, Survey};
use crate::model::generated::{answer, survey};
use crate::model::judge::get_judge_result;
use crate::service::token::TokenInfo;
use crate::DATABASE;
use axum::Json;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, FromQueryResult, NotSet, QuerySelect, SelectColumns};
use sea_orm::{EntityTrait, IntoActiveModel, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

pub async fn submit_answer(TokenInfo(user): TokenInfo, Json(request): Json<SubmitRequest>) -> Result<String, ErrorMessage> {
    let complete = request.complete.unwrap_or(false);
    if complete { 
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
            answers: NotSet,
            score: NotSet,
            create_time: NotSet,
            completed: NotSet,
        }
    };
    
    for (key, value) in request.content {
        if !value.is_empty() { 
            new_answer[key] = value.into();
        }
    }
    
    // let answer_object = new_answer.as_object().ok_or(ErrorMessage::InvalidField)?;
    
    // if complete {
    //     let survey = Survey::find()
    //         .filter(survey::Column::Id.eq(request.survey))
    //         .one(&*DATABASE)
    //         .await.unwrap()
    //         .ok_or(ErrorMessage::NotFound)?;
    // 
    //     let mut page = get_page_by_id(&survey.page).await;
    // 
    //     while let Some(ref page_info) = page {
    //         for question in &page_info.content {
    //             let question = get_question_by_id(question).await.unwrap();
    //             
    //             if question.required && !answer_object.contains_key(&question.id) {
    //                 return Err(ErrorMessage::MissingField);
    //             }
    //         }
    //         
    //         page = if let Some(ref next) = page_info.next {
    //             get_page_by_id(next).await
    //         } else {
    //             None
    //         };
    //     }
    // }
    
    answer.answers = Set(new_answer);
    answer.completed = Set(request.complete.unwrap_or(false));
    
    let ret = answer.save(&*DATABASE).await.unwrap();
    let id = ret.id.unwrap();
    
    if complete {
        get_judge_result(id, 0).await?;
    }

    Ok(id.to_string())
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
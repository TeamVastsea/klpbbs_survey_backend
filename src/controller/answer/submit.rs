use crate::controller::error::ErrorMessage;
use crate::model::generated::answer;
use crate::model::generated::prelude::Answer;
use crate::service::token::TokenInfo;
use crate::DATABASE;
use axum::Json;
use sea_orm::{ActiveModelTrait, ColumnTrait, NotSet};
use sea_orm::{EntityTrait, IntoActiveModel, QueryFilter};
use serde::Deserialize;
use std::collections::HashMap;
use sea_orm::ActiveValue::Set;
use serde_json::json;

pub async fn submit_answer(TokenInfo(user): TokenInfo, Json(request): Json<SubmitRequest>) -> Result<String, ErrorMessage> {
    let mut new_answer = json!({});

    let mut answer = if let Some(old_answer_id) = request.id {
        let old = Answer::find()
            .filter(answer::Column::Id.eq(old_answer_id))
            .filter(answer::Column::User.eq(user.uid.parse::<i64>().unwrap()))
            .filter(answer::Column::Survey.eq(&request.survey))
            .filter(answer::Column::Score.is_null())
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
        }
    };
    
    for (key, value) in request.content {
        if !value.is_empty() { 
            new_answer[key] = value.into();
        }
    }
    
    answer.answers = Set(new_answer);
    
    let ret = answer.save(&*DATABASE).await.unwrap();

    Ok(ret.id.unwrap().to_string())
}

#[derive(Deserialize)]
pub struct SubmitRequest {
    id: Option<i32>,
    survey: String,
    content: HashMap<String, String>,
}
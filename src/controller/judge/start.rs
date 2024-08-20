use std::collections::HashMap;
use crate::model::generated::prelude::{Answer, Page, Survey};
use axum::extract::Query;
use sea_orm::ColumnTrait;
use sea_orm::{EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;
use crate::controller::error::ErrorMessage;
use crate::DATABASE;
use crate::model::question::Question;
use crate::service::admin::AdminTokenInfo;
use crate::service::judge::judge_subjectives;
use crate::service::questions::get_question_by_id;

pub async fn auto_judge(Query(query): Query<JudgeRequest>, AdminTokenInfo(admin): AdminTokenInfo) -> Result<String, ErrorMessage> {
    info!("Admin {} is judging survey {}", admin.id, query.survey);
    
    let answer = Answer::find()
        .filter(crate::model::generated::answer::Column::Id.eq(query.answer))
        .one(&*DATABASE).await.unwrap().unwrap();
    let answers = serde_json::from_value::<HashMap<String, String>>(answer.answers).unwrap();

    let survey = Survey::find()
        .filter(crate::model::generated::survey::Column::Id.eq(query.survey))
        .one(&*DATABASE).await.unwrap().unwrap();

    let mut questions = Vec::new();

    let mut page = Some(survey.page);
    while let Some(p) = page {
        let p = p.clone();
        let p = Page::find()
            .filter(crate::model::generated::page::Column::Id.eq(p))
            .one(&*DATABASE).await.unwrap().unwrap();

        for question_id in p.content {
            questions.push(get_question_by_id(&question_id).await.ok_or(ErrorMessage::NotFound)?);
        }

        page = p.next;
    }

    let questions = questions.iter().map(|q| q.clone().try_into().unwrap()).collect::<Vec<Question>>();

    let (full, user, scores) = judge_subjectives(&questions, &answers).await;

    let response = JudgeResponse {
        full,
        user,
        scores,
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[derive(Deserialize)]
pub struct JudgeRequest {
    pub survey: i32,
    pub answer: i32,
}

#[derive(Serialize)]
pub struct JudgeResponse {
    pub full: i32,
    pub user: i32,
    pub scores: HashMap<Uuid, i32>,
}
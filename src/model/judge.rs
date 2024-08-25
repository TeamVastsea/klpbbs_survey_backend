use crate::controller::error::ErrorMessage;
use crate::model::generated::prelude::{Answer, Page, Survey};
use crate::model::generated::score;
use crate::model::question::Question;
use crate::service::judge::judge_subjectives;
use crate::service::questions::get_question_by_id;
use crate::DATABASE;
use chrono::{NaiveDateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, NotSet};
use sea_orm::{EntityTrait, QueryFilter};
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn get_judge_result(answer: i32, judge: i64) -> Result<JudgeResult, ErrorMessage> {
    let score = score::Entity::find()
        .filter(score::Column::Id.eq(answer))
        .one(&*DATABASE)
        .await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;

    if let Some(res) = score {
        let score: HashMap<Uuid, i32> = serde_json::from_value(res.scores).unwrap();

        return Ok(JudgeResult {
            full: res.full_score,
            user: res.user_score,
            scores: score,
            completed: res.completed,
            judge: res.judge,
            judge_time: res.judge_time,
        });
    }

    let result = auto_judge(answer, judge).await?;

    Ok(result)
}

async fn auto_judge(answer: i32, judge: i64) -> Result<JudgeResult, ErrorMessage> {
    let answer = Answer::find()
        .filter(crate::model::generated::answer::Column::Id.eq(answer))
        .one(&*DATABASE).await.unwrap().unwrap();
    let answers = serde_json::from_value::<HashMap<String, String>>(answer.answers).unwrap();

    let survey = Survey::find()
        .filter(crate::model::generated::survey::Column::Id.eq(answer.survey))
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

    save_judge_result(&scores, answer.user, judge, answer.id, user, full, false).await?;

    Ok(JudgeResult {
        full,
        user,
        scores,
        completed: false,
        judge,
        judge_time: Utc::now().naive_utc(),
    })
}

async fn save_judge_result(
    judge_result: &HashMap<Uuid, i32>,
    user_id: i64,
    judge_id: i64,
    answer_id: i32,
    user_score: i32,
    full_score: i32,
    exist: bool,
) -> Result<(), ErrorMessage> {
    let scores = score::ActiveModel {
        id: Set(answer_id),
        user: Set(user_id),
        judge: Set(judge_id),
        judge_time: Set(Utc::now().naive_utc()),
        scores: Set(serde_json::to_value(judge_result).unwrap()),
        user_score: Set(user_score),
        full_score: Set(full_score),
        completed: NotSet,
    };

    if exist {
        scores.update(&*DATABASE).await.map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;
    } else {
        scores.insert(&*DATABASE).await.map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;
    }

    Ok(())
}

#[derive(Serialize)]
pub struct JudgeResult {
    pub full: i32,
    pub user: i32,
    pub scores: HashMap<Uuid, i32>,
    pub completed: bool,
    pub judge: i64,
    pub judge_time: NaiveDateTime,
}
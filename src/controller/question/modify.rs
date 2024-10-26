use crate::controller::error::ErrorMessage;
use crate::dao::entity::question;
use crate::dao::model::question::{NewQuestion, Question};
use axum::Json;
use sea_orm::{ActiveModelTrait, IntoActiveModel};
use serde::Deserialize;

pub async fn new_question(Json(question): Json<NewQuestion>) -> Result<String, ErrorMessage> {
    let question = Question::create(question).await
        .ok_or(ErrorMessage::DatabaseError("Failed to create question".to_string()))?;

    Ok(question.id.to_string())
}

pub async fn modify_question(Json(question): Json<question::Model>) -> String {
    let result = question.into_active_model().reset_all().update(&*crate::DATABASE).await.unwrap();

    result.id.to_string()
}

pub async fn swap_question(Json(question): Json<SwapQuestionRequest>) -> Result<String, ErrorMessage> {
    Question::change_position(question.from, question.to).await;

    Ok("".to_string())
}

#[derive(Deserialize)]
pub struct SwapQuestionRequest {
    from: i32,
    to: i32,
}
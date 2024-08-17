use crate::model::question::QuestionType;
use crate::service::questions::save_question;
use axum::Json;
use sea_orm::JsonValue;
use serde::Serialize;

pub async fn new_question(Json(question): Json<NewQuestionRequest>) -> String {
    let id = save_question(question.content, question.r#type, question.values, question.condition, question.required, None).await;
    id.to_string()
}

#[derive(serde::Deserialize, Serialize)]
pub struct NewQuestionRequest {
    pub content: JsonValue,
    pub r#type: QuestionType,
    pub values: Option<Vec<JsonValue>>,
    pub condition: Option<String>,
    pub required: bool,
}
use crate::model::question::{Condition, QuestionType};
use crate::service::questions::save_question;
use axum::Json;
use serde::Serialize;
use crate::model::ValueWithTitle;

pub async fn new_question(Json(question): Json<NewQuestionRequest>) -> String {
    let content = serde_json::to_value(question.content).unwrap();
    let values = question.values.map(|values| 
        values.iter().map(|v| serde_json::to_value(v).unwrap()).collect());
    let condition = question.condition.map(|c| serde_json::to_string(&c).unwrap());
    
    let id = save_question(content, question.r#type, values, condition, question.required, None).await;
    id.to_string()
}

#[derive(serde::Deserialize, Serialize)]
pub struct NewQuestionRequest {
    pub content: ValueWithTitle,
    pub r#type: QuestionType,
    pub values: Option<Vec<ValueWithTitle>>,
    pub condition: Option<Vec<Condition>>,
    pub required: bool,
}
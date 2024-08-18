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
    
    let id = save_question(content, question.r#type, values, condition, question.required,
                           None, question.all_points, question.sub_points, question.answer).await;
    id.to_string()
}

pub async fn modify_question(Json(question): Json<ModifyQuestionRequest>) -> String {
    let content = serde_json::to_value(question.content).unwrap();
    let values = question.values.map(|values| 
        values.iter().map(|v| serde_json::to_value(v).unwrap()).collect());
    let condition = question.condition.map(|c| serde_json::to_string(&c).unwrap());
    
    let id = save_question(content, question.r#type, values, condition, question.required, Some(question.id.clone()), 
                           question.all_points, question.sub_points, question.answer).await;
    id.to_string()
}

#[derive(serde::Deserialize, Serialize)]
pub struct NewQuestionRequest {
    pub content: ValueWithTitle,
    pub r#type: QuestionType,
    pub values: Option<Vec<ValueWithTitle>>,
    pub condition: Option<Vec<Condition>>,
    pub required: bool,
    pub all_points: i32,
    pub sub_points: Option<i32>,
    pub answer: Option<String>,
}

#[derive(serde::Deserialize, Serialize)]
pub struct ModifyQuestionRequest {
    pub id: String,
    pub content: ValueWithTitle,
    pub r#type: QuestionType,
    pub values: Option<Vec<ValueWithTitle>>,
    pub condition: Option<Vec<Condition>>,
    pub required: bool,
    pub all_points: i32,
    pub sub_points: Option<i32>,
    pub answer: Option<String>,
}
use crate::controller::error::ErrorMessage;
use crate::dao::model::question::{NewQuestion, Question};
use crate::service::token::AdminTokenInfo;
use axum::Json;
use sea_orm::{ActiveModelTrait, IntoActiveModel};
use serde::Deserialize;
use tracing::info;

pub async fn new_question(AdminTokenInfo(user): AdminTokenInfo, Json(question): Json<NewQuestion>) -> Result<String, ErrorMessage> {
    info!("Admin {} create question", user.uid);
    let question = Question::create(question).await?;

    Ok(question.id.to_string())
}

pub async fn modify_question(AdminTokenInfo(user): AdminTokenInfo, Json(question): Json<Question>) -> String {
    info!("Admin {} modify question {}", user.uid, question.id);
    let result = question.to_entity().into_active_model().reset_all().update(&*crate::DATABASE).await.unwrap();

    result.id.to_string()
}

pub async fn swap_question(AdminTokenInfo(user): AdminTokenInfo, Json(question): Json<SwapQuestionRequest>) -> Result<String, ErrorMessage> {
    info!("Admin {} swap question {} and {} of page {}", user.uid, question.from, question.to, question.page);
    Question::change_position(question.page, question.from, question.to).await?;

    Ok("".to_string())
}

#[derive(Deserialize)]
pub struct SwapQuestionRequest {
    page: i32,
    from: i32,
    to: i32,
}
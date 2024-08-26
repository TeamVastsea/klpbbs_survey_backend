use crate::controller::error::ErrorMessage;
use crate::service::admin::AdminTokenInfo;
use crate::service::questions::get_question_by_id;
use crate::service::token::TokenInfo;
use axum::extract::Path;
use tracing::info;

pub async fn get_question(Path(question): Path<String>, TokenInfo(user): TokenInfo) -> Result<String, ErrorMessage> {
    info!("User {} is trying to get question {}", user.uid, question);
    
    let Some(mut question) = get_question_by_id(&question).await
    else {
        return Err(ErrorMessage::NotFound);
    };
    
    question.answer = None;
    question.all_points = None;
    question.sub_points = None;

    Ok(serde_json::to_string(&question).unwrap())
}

pub async fn get_admin_question(Path(question): Path<String>, AdminTokenInfo(admin): AdminTokenInfo) -> Result<String, ErrorMessage> {
    info!("Admin {} is trying to get question {}", admin.id, question);
    
    let Some(page) = get_question_by_id(&question).await
    else {
        return Err(ErrorMessage::NotFound);
    };

    Ok(serde_json::to_string(&page).unwrap())
}
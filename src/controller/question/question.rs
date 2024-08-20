use crate::controller::error::ErrorMessage;
use crate::service::questions::get_question_by_id;
use crate::service::token::TokenInfo;
use axum::extract::Path;
use tracing::info;
use crate::service::admin::AdminTokenInfo;

pub async fn get_question(Path(question): Path<String>, TokenInfo(user): TokenInfo) -> Result<String, ErrorMessage> {
    info!("User {} is trying to get question {}", user.uid, question);
    
    let Some(mut page) = get_question_by_id(&question).await
    else {
        return Err(ErrorMessage::NotFound);
    };
    
    page.answer = None;
    page.all_points = None;
    page.sub_points = None;

    Ok(serde_json::to_string(&page).unwrap())
}

pub async fn get_admin_question(Path(question): Path<String>, AdminTokenInfo(admin): AdminTokenInfo) -> Result<String, ErrorMessage> {
    info!("Admin {} is trying to get question {}", admin.id, question);
    
    let Some(page) = get_question_by_id(&question).await
    else {
        return Err(ErrorMessage::NotFound);
    };

    Ok(serde_json::to_string(&page).unwrap())
}
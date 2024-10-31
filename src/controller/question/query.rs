use crate::controller::error::ErrorMessage;
use crate::dao::model::question::Question;
use crate::service::token::TokenInfo;
use axum::extract::Query;
use serde::Deserialize;
use tracing::info;

// pub async fn get_question(Path(question): Path<i32>, TokenInfo(user): TokenInfo) -> Result<String, ErrorMessage> {
//     info!("User {} is trying to get question {}", user.uid, question);
// 
//     let Ok(question) = Question::find_by_id(question).await
//     else {
//         return Err(ErrorMessage::NotFound);
//     };
//     let mut question = question.to_modal()?;
// 
//     if !user.admin {
//         if !question.get_access().await? {
//             return Err(ErrorMessage::PermissionDenied);
//         }
// 
//         question.answer = None;
//     }
// 
//     question.answer = None;
// 
//     Ok(serde_json::to_string(&question).unwrap())
// }

pub async fn get_question_by_page(Query(query): Query<GetQuestionPageRequest>, TokenInfo(user): TokenInfo) -> Result<String, ErrorMessage> {
    info!("User {}(admin: {}) is trying to get questions from page {}", user.uid, user.admin, query.page);

    let mut questions = Question::find_by_page(query.page).await?;

    if !user.admin {
        let Some(first) = questions.first() else { return Ok("[]".to_string()); };
        let (access, source) = first.get_access().await?;
        if !access || source.is_some_and(|s| s != user.source) {
            return Err(ErrorMessage::PermissionDenied);
        }

        for question in &mut questions {
            question.answer = None;
        }
    }

    Ok(serde_json::to_string(&questions).unwrap())
}

#[derive(Deserialize)]
pub struct GetQuestionPageRequest {
    pub page: i32,
}
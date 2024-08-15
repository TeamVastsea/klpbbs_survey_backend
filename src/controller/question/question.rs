use crate::controller::error::ErrorMessage;
use crate::model::generated::prelude::Question;
use crate::model::generated::question;
use crate::DATABASE;
use axum::extract::Path;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::prelude::Uuid;
use tracing::info;
use crate::service::token::TokenInfo;

pub async fn get_question(Path(question): Path<String>, TokenInfo(user): TokenInfo) -> Result<String, ErrorMessage> {
    info!("User {} is trying to get question {}", user.uid, question);
    
    let Some(page) = Question::find().filter(question::Column::Id.eq(question)).one(&*DATABASE).await.unwrap()
    else {
        return Err(ErrorMessage::NotFound);
    };

    Ok(serde_json::to_string(&page).unwrap())
}

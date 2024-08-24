use crate::controller::error::ErrorMessage;
use crate::model::generated::answer;
use crate::model::generated::prelude::Answer;
use crate::service::admin::AdminTokenInfo;
use crate::DATABASE;
use axum::extract::Path;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use tracing::info;

pub async fn get_answer(Path(id): Path<i32>, AdminTokenInfo(admin): AdminTokenInfo) -> Result<String, ErrorMessage> {
    info!("Admin {} get answer {}", admin.id, id);
    
    let answer = Answer::find().filter(answer::Column::Id.eq(id)).one(&*DATABASE).await.unwrap()
        .ok_or(ErrorMessage::NotFound)?;
    Ok(serde_json::to_string(&answer).unwrap())
}
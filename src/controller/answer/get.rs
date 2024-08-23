use axum::extract::Path;
use axum::Json;
use sea_orm::EntityTrait;
use tracing::info;
use crate::controller::error::ErrorMessage;
use crate::DATABASE;
use crate::model::generated::answer;
use crate::model::generated::prelude::Answer;
use crate::service::admin::AdminTokenInfo;
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;

pub async fn get_answer(Path(id): Path<i32>, AdminTokenInfo(admin): AdminTokenInfo) -> Result<String, ErrorMessage> {
    info!("Admin {} get answer {}", admin.id, id);
    
    let answer = Answer::find().filter(answer::Column::Id.eq(id)).one(&*DATABASE).await.unwrap()
        .ok_or(ErrorMessage::NotFound)?;
    Ok(serde_json::to_string(&answer).unwrap())
}
use crate::controller::error::ErrorMessage;
use crate::model::generated::answer;
use crate::model::generated::prelude::Answer;
use crate::service::token::TokenInfo;
use crate::DATABASE;
use axum::extract::Query;
use sea_orm::ColumnTrait;
use sea_orm::{EntityTrait, QueryFilter};
use serde::Deserialize;
use serde_json::json;

pub async fn check_record(Query(query): Query<CheckQuery>, TokenInfo(user): TokenInfo) -> Result<String, ErrorMessage> {
    let answers = if query.only_unfinished {
        Answer::find()
        .filter(answer::Column::User.eq(user.uid.parse::<u64>().unwrap()))
        .filter(answer::Column::Survey.eq(query.survey))
        .filter(answer::Column::Completed.eq(false))
        .all(&*DATABASE)
        .await
        .unwrap()
    } else { 
        Answer::find()
            .filter(answer::Column::User.eq(user.uid.parse::<u64>().unwrap()))
            .filter(answer::Column::Survey.eq(query.survey))
            .all(&*DATABASE)
            .await
            .unwrap()
    };

    let records = answers.iter().map(|rec| json!({"id": rec.id, "time": rec.create_time})).collect::<Vec<_>>();

    Ok(serde_json::to_string(&records).unwrap())
}

#[derive(Deserialize)]
pub struct CheckQuery {
    pub only_unfinished: bool,
    pub survey: String,
}
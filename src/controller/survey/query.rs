use crate::model::generated::prelude::Survey;
use crate::model::generated::survey;
use crate::DATABASE;
use axum::extract::{Path, Query};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use tracing::debug;
use crate::service::token::TokenInfo;

pub async fn query_surveys(Query(query): Query<QueryParams>, TokenInfo(user): TokenInfo) -> String {
    debug!("User {} is trying to get surveys", user.uid);
    
    let size = query.size.unwrap_or(10);
    let page = query.page.unwrap_or(1);

    let current_time = chrono::Local::now().naive_local();


    let paginator = if let Some(search) = query.search {
        let search = format!("%{}%", search);
        Survey::find()
            .filter(survey::Column::Title.like(&search)
            .or(survey::Column::Description.like(&search)))
            .filter(survey::Column::StartDate.lte(current_time))
            .filter(survey::Column::EndDate.gte(current_time))
            .paginate(&*DATABASE, size)
    } else {
        Survey::find()
            .filter(survey::Column::StartDate.lte(current_time))
            .filter(survey::Column::EndDate.gte(current_time))
            .paginate(&*DATABASE, size)
    };


    let surveys = paginator.fetch_page(page).await.unwrap();

    serde_json::to_string(&surveys).unwrap()
}

pub async fn query_by_id(Path(id): Path<i32>) -> String {
    let survey = Survey::find().filter(survey::Column::Id.eq(id)).one(&*DATABASE).await.unwrap();

    serde_json::to_string(&survey).unwrap()
}

#[derive(serde::Deserialize)]
pub struct QueryParams {
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub search: Option<String>,
}
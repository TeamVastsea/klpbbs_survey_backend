use crate::controller::error::ErrorMessage;
use crate::dao::entity::prelude::Survey;
use crate::dao::entity::survey;
use crate::service::token::TokenInfo;
use crate::DATABASE;
use axum::extract::{Path, Query};
use migration::Order;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use tracing::debug;

pub async fn query_surveys(Query(query): Query<QueryParams>, TokenInfo(user): TokenInfo) -> String {
    let admin = user.admin;

    debug!("User {} (admin: {admin}) is trying to get surveys", user.uid);

    let size = query.size.unwrap_or(10);
    let page = query.page.unwrap_or(0);

    let current_time = chrono::Local::now().naive_local();

    let mut select = Survey::find();

    if !admin {
        select = select.filter(survey::Column::AllowSubmit.eq(true))
            .filter(survey::Column::AllowView.eq(true))
            .filter(survey::Column::StartDate.lte(current_time))
            .filter(survey::Column::EndDate.gte(current_time));
    }

    if let Some(search) = query.search {
        let search = format!("%{}%", search);
        select = select.filter(survey::Column::Title.like(search));
    }

    let paginator = select.paginate(&*DATABASE, size);


    let surveys = paginator.fetch_page(page).await.unwrap();

    serde_json::to_string(&surveys).unwrap()
}

pub async fn query_by_id(Path(id): Path<i32>, TokenInfo(user): TokenInfo) -> Result<String, ErrorMessage> {
    let admin = user.admin;

    debug!("User {} (admin: {admin}) is trying to get surveys", user.uid);

    let mut select = Survey::find()
        .filter(survey::Column::Id.eq(id))
        .order_by(survey::Column::Id, Order::Asc);

    if !admin {
        select = select.filter(survey::Column::AllowSubmit.eq(true))
            .filter(survey::Column::StartDate.lte(chrono::Local::now().naive_local()))
            .filter(survey::Column::EndDate.gte(chrono::Local::now().naive_local()));
    }

    let survey = select.one(&*DATABASE).await.unwrap().ok_or(ErrorMessage::NotFound)?;

    Ok(serde_json::to_string(&survey).unwrap())
}

#[derive(serde::Deserialize)]
pub struct QueryParams {
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub search: Option<String>,
}
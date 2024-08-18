use sea_orm::ColumnTrait;
use lazy_static::lazy_static;
use moka::future::Cache;
use sea_orm::{ActiveModelTrait, EntityTrait, JsonValue, QueryFilter};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;
use crate::DATABASE;
use crate::model::generated::{page, question};
use crate::model::generated::prelude::{Page, Question};
use crate::model::question::QuestionType;

lazy_static! {
    static ref QUESTIO_CACHE: Cache<String, question::Model> = Cache::new(10000);
    static ref PAHE_CACHE: Cache<String, page::Model> = Cache::new(10000);
}

pub async fn get_question_by_id(id: &str) -> Option<question::Model> {
    if let Some(a) = QUESTIO_CACHE.get(id).await {
        return Some(a);
    }

    let question = Question::find()
        .filter(question::Column::Id.eq(id))
        .one(&*crate::DATABASE).await.unwrap()?;

    QUESTIO_CACHE.insert(id.to_string(), question.clone()).await;

    Some(question)
}

pub async fn get_page_by_id(id: &str) -> Option<page::Model> {
    if let Some(a) = PAHE_CACHE.get(id).await {
        return Some(a);
    }

    let page = Page::find()
        .filter(page::Column::Id.eq(id))
        .one(&*crate::DATABASE).await.unwrap()?;

    PAHE_CACHE.insert(id.to_string(), page.clone()).await;

    Some(page)
}

pub async fn save_question(content: JsonValue,
                           question_type: QuestionType,
                           values: Option<Vec<JsonValue>>,
                           condition: Option<String>,
                           required: bool,
                           id: Option<String>,
                           all_points: i32, 
                           sub_points: Option<i32>,
                           answer: Option<String>) -> String {
    let id_generate = id.clone().unwrap_or(Uuid::new_v4().to_string());

    let question = question::ActiveModel {
        id: Set(id_generate.clone()),
        content: Set(content),
        r#type: Set(question_type as i32),
        values: Set(values),
        condition: Set(condition),
        required: Set(required),
        all_points: Set(all_points),
        sub_points: Set(sub_points),
        answer: Set(answer),
    };

    let after = if id.clone().is_some() {
        question.update(&*DATABASE).await.unwrap()
    } else {
        question.insert(&*DATABASE).await.unwrap()
    };

    QUESTIO_CACHE.insert(id_generate.clone(), after.clone()).await;

    id_generate
}

pub async fn save_page(title: String, content: Vec<String>, next: Option<String>, id: Option<String>) -> String {
    let id_generate = id.clone().unwrap_or(Uuid::new_v4().to_string());

    let page = page::ActiveModel {
        id: Set(id_generate.clone()),
        title: Set(title),
        content: Set(content),
        next: Set(next),
    };

    let after = if id.is_some() {
        page.update(&*DATABASE).await.unwrap()
    } else {
        page.insert(&*DATABASE).await.unwrap()
    };

    PAHE_CACHE.insert(id_generate.clone(), after.clone()).await;

    id_generate
}
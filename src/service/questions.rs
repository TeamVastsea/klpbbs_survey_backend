use crate::model::generated::prelude::{Page, Question};
use crate::model::generated::{page, question};
use crate::model::question::QuestionType;
use crate::DATABASE;
use lazy_static::lazy_static;
use moka::future::Cache;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::{ActiveModelTrait, EntityTrait, JsonValue, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

lazy_static! {
    static ref QUESTIO_CACHE: Cache<String, question::Model> = Cache::new(10000);
    static ref PAGE_CACHE: Cache<String, page::Model> = Cache::new(10000);
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
    if let Some(a) = PAGE_CACHE.get(id).await {
        return Some(a);
    }

    let page = Page::find()
        .filter(page::Column::Id.eq(id))
        .one(&*crate::DATABASE).await.unwrap()?;

    PAGE_CACHE.insert(id.to_string(), page.clone()).await;

    Some(page)
}

pub async fn save_question(content: JsonValue,
                           question_type: QuestionType,
                           values: Option<Vec<JsonValue>>,
                           condition: Option<String>,
                           required: bool,
                           id: Option<String>,
                           answer: Option<crate::model::question::Answer>) -> String {
    let id_generate = id.clone().unwrap_or(Uuid::new_v4().to_string());

    let question = question::ActiveModel {
        id: Set(id_generate.clone()),
        content: Set(content),
        r#type: Set(question_type as i32),
        values: Set(values),
        condition: Set(condition),
        required: Set(required),
        all_points: Set(answer.as_ref().map(|a| a.all_points)),
        sub_points: Set(answer.as_ref().and_then(|a| a.sub_points)),
        answer: Set(answer.map(|a| a.answer)),
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

    PAGE_CACHE.insert(id_generate.clone(), after.clone()).await;

    id_generate
}

pub fn refresh_cache(refresh_type: CacheType) {
    match refresh_type {
        CacheType::Question => {
            QUESTIO_CACHE.invalidate_all();
        }
        CacheType::Page => {
            PAGE_CACHE.invalidate_all();
        }
        CacheType::Both => {
            QUESTIO_CACHE.invalidate_all();
            PAGE_CACHE.invalidate_all();
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CacheType {
    Question,
    Page,
    Both
}
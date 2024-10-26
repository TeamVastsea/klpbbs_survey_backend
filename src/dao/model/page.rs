use crate::controller::error::ErrorMessage;
use crate::dao::entity::page;
use crate::dao::entity::prelude::{Page, Survey};
use crate::DATABASE;
use lazy_static::lazy_static;
use moka::future::Cache;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, NotSet, QueryFilter};
use sea_orm::{ColumnTrait, PaginatorTrait, QueryOrder};
use uuid::Uuid;

lazy_static! {
    pub static ref PAGE_CACHE: Cache<String, page::Model> = Cache::new(10000);
}

impl page::Model {
    pub async fn find_by_id(id: &str) -> Result<Self, ErrorMessage> {
        if let Some(a) = PAGE_CACHE.get(id).await {
            return Ok(a);
        }

        let page = Page::find()
            .filter(page::Column::Id.eq(Uuid::parse_str(id).map_err(|_| ErrorMessage::InvalidField {
                field: String::from("id"),
                should_be: String::from("uuid")
            })?))
            .one(&*DATABASE).await.unwrap()
            .ok_or(ErrorMessage::NotFound)?;

        PAGE_CACHE.insert(id.to_string(), page.clone()).await;

        Ok(page)
    }
    
    pub async fn get_by_survey_and_index(survey: &str, index: u64) -> Result<(Self, u64), ErrorMessage> {
        let page = Page::find()
            .filter(page::Column::Survey.eq(Uuid::parse_str(survey).map_err(|_| ErrorMessage::InvalidField {
                field: String::from("survey"),
                should_be: String::from("uuid")
            })?))
            .order_by_asc(page::Column::Order)
            .paginate(&*DATABASE, 1);
        
        let content = page.fetch_page(index).await.unwrap().pop().ok_or(ErrorMessage::NotFound)?;
        let page = page.num_pages().await.map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;

        Ok((content, page))
    }
    
    pub async fn check_access(&self) -> Result<(bool, bool, bool), ErrorMessage> {
        let survey = self.find_related(Survey).one(&*DATABASE).await.unwrap().ok_or(ErrorMessage::NotFound)?;
        
        Ok((survey.allow_submit, survey.allow_view, survey.allow_re_submit))
    }
    
    pub async fn new_page(title: String, survey: i32) -> Self {
        let page = page::ActiveModel {
            id: Set(Uuid::new_v4()),
            title: Set(title),
            order: NotSet,
            survey: Set(survey),
        };
        
        page.insert(&*DATABASE).await.unwrap()
    }
    
    pub async fn update(id: &str, title: String, order: i32) -> Self {
        PAGE_CACHE.invalidate(id).await;
        
        let page = page::ActiveModel {
            id: Set(Uuid::parse_str(id).unwrap()),
            title: Set(title),
            order: Set(order),
            survey: NotSet,
        };
        
        page.update(&*DATABASE).await.unwrap()
    }
    
    pub async fn delete(id: &str) {
        PAGE_CACHE.invalidate(id).await;
        
        let page = page::ActiveModel {
            id: Set(id.parse().unwrap()),
            title: NotSet,
            order: NotSet,
            survey: NotSet,
        };
        
        page.delete(&*DATABASE).await.unwrap();
    }
}

// pub async fn get_page_by_id(id: &str) -> Option<page::Model> {
//     if let Some(a) = PAGE_CACHE.get(id).await {
//         return Some(a);
//     }
// 
//     let page = Page::find()
//         .filter(page::Column::Id.eq(id))
//         .one(&*crate::DATABASE).await.unwrap()?;
// 
//     PAGE_CACHE.insert(id.to_string(), page.clone()).await;
// 
//     Some(page)
// }
// 
// pub async fn save_question(content: JsonValue,
//                            question_type: QuestionType,
//                            values: Option<Vec<JsonValue>>,
//                            condition: Option<String>,
//                            required: bool,
//                            id: Option<String>,
//                            answer: Option<crate::model::question::Answer>) -> String {
//     let id_generate = id.clone().unwrap_or(Uuid::new_v4().to_string());
// 
//     let question = question::ActiveModel {
//         id: Set(id_generate.clone()),
//         content: Set(content),
//         r#type: Set(question_type as i32),
//         values: Set(values),
//         condition: Set(condition),
//         required: Set(required),
//         all_points: Set(answer.as_ref().map(|a| a.all_points)),
//         sub_points: Set(answer.as_ref().and_then(|a| a.sub_points)),
//         answer: Set(answer.map(|a| a.answer)),
//     };
// 
//     let after = if id.clone().is_some() {
//         question.update(&*DATABASE).await.unwrap()
//     } else {
//         question.insert(&*DATABASE).await.unwrap()
//     };
// 
//     QUESTION_CACHE.insert(id_generate.clone(), after.clone()).await;
// 
//     id_generate
// }
// 
// pub async fn save_page(title: String, content: Vec<String>, next: Option<String>, previous: Option<String>, id: Option<String>) -> String {
//     let id_generate = id.clone().unwrap_or(Uuid::new_v4().to_string());
// 
//     let page = page::ActiveModel {
//         id: Set(id_generate.clone()),
//         title: Set(title),
//         content: Set(content),
//         next: Set(next),
//         previous: Set(previous),
//     };
// 
//     let after = if id.is_some() {
//         page.update(&*DATABASE).await.unwrap()
//     } else {
//         page.insert(&*DATABASE).await.unwrap()
//     };
// 
//     PAGE_CACHE.insert(id_generate.clone(), after.clone()).await;
// 
//     id_generate
// }
use crate::controller::error::ErrorMessage;
use crate::dao::entity::page;
use crate::dao::entity::prelude::{Page, Survey};
use crate::DATABASE;
use futures::StreamExt;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait, NotSet, QueryFilter, QuerySelect};
use sea_orm::{ColumnTrait, PaginatorTrait, QueryOrder};
use crate::dao::entity::user::UserType;

impl page::Model {
    pub async fn find_by_id(id: i32) -> Result<Self, ErrorMessage> {
        let page = Page::find()
            .filter(page::Column::Id.eq(id))
            .one(&*DATABASE).await.unwrap()
            .ok_or(ErrorMessage::NotFound)?;

        Ok(page)
    }

    pub async fn get_by_survey_and_index(survey: i32, index: u64) -> Result<(Self, u64), ErrorMessage> {
        let page = Page::find()
            .filter(page::Column::Survey.eq(survey))
            .order_by_asc(page::Column::Id)
            .paginate(&*DATABASE, 1);

        let content = page.fetch_page(index).await.unwrap().pop().ok_or(ErrorMessage::NotFound)?;
        let page = page.num_pages().await.map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;

        Ok((content, page))
    }

    pub async fn check_access(&self) -> Result<(bool, bool, bool, Option<UserType>), ErrorMessage> {
        let survey = self.find_related(Survey).one(&*DATABASE).await.unwrap().ok_or(ErrorMessage::NotFound)?;

        Ok((survey.allow_submit, survey.allow_view, survey.allow_re_submit, survey.user_source))
    }

    pub async fn new_page(title: String, survey: i32, index: i32) -> Self {
        let mut pages = Page::find()
            .filter(page::Column::Survey.eq(survey))
            .order_by_asc(page::Column::Id)
            .offset((index + 1) as u64)
            .stream(&*DATABASE).await.unwrap();

        let Some(first) = pages.next().await else {
            let page = page::ActiveModel {
                id: NotSet,
                title: Set(title),
                survey: Set(survey),
            };

            return page.insert(&*DATABASE).await.unwrap();
        };
        let first = first.unwrap();
        let mut last = first.title.clone();

        while let Some(page) = pages.next().await {
            let page = page.unwrap();

            let mut changes = page.clone().into_active_model();
            changes.title = Set(last);
            changes.update(&*DATABASE).await.unwrap();

            last = page.title;
        }

        let last = page::ActiveModel {
            id: NotSet,
            title: Set(last),
            survey: Set(survey),
        };
        last.insert(&*DATABASE).await.unwrap();

        let mut created_page = first.into_active_model();
        created_page.title = Set(title);
        created_page.update(&*DATABASE).await.unwrap()
    }

    pub async fn update(id: i32, title: String) -> Self {
        let page = page::ActiveModel {
            id: Set(id),
            title: Set(title),
            survey: NotSet,
        };

        page.update(&*DATABASE).await.unwrap()
    }

    pub async fn delete(id: i32) {
        let page = page::ActiveModel {
            id: Set(id),
            title: NotSet,
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
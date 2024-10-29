use futures::StreamExt;
use crate::controller::error::ErrorMessage;
use crate::dao::entity::question::QuestionType;
use crate::dao::entity::{page, question};
use crate::dao::model::ValueWithTitle;
use lazy_static::lazy_static;
use moka::future::Cache;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, NotSet, Order, PaginatorTrait, QueryOrder, QuerySelect};
use sea_orm::{ColumnTrait, JsonValue};
use sea_orm::{QueryFilter};
use serde::{Deserialize, Serialize};
use crate::DATABASE;

impl Question {
    pub async fn find_by_id(id: i32) -> Result<question::Model, ErrorMessage> {

        let question = question::Entity::find()
            .filter(question::Column::Id.eq(id))
            .one(&*crate::DATABASE).await.unwrap()
            .ok_or(ErrorMessage::NotFound)?;

        Ok(question)
    }

    pub async fn find_by_page(page_id: i32) -> Result<Vec<Self>, ErrorMessage> {
        let questions = question::Entity::find()
            .filter(question::Column::Page.eq(page_id))
            .order_by_asc(question::Column::Id)
            .all(&*crate::DATABASE).await.unwrap();

        questions.iter().map(|q| q.clone().to_modal()).collect::<Result<Vec<_>, _>>()
    }

    pub async fn update(&self) {
        let entity = self.to_entity().into_active_model().reset_all();
        entity.update(&*crate::DATABASE).await.unwrap();
    }

    pub async fn change_position(page: i32, from: i32, to: i32) -> Result<(), ErrorMessage> {
        let total = question::Entity::find()
            .filter(question::Column::Page.eq(page))
            .count(&*DATABASE).await
            .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))? as i32;

        let (offset, limit, order) = if from < to {
            (from, to - from + 1, Order::Asc)
        } else {
            (total - from - 1, from - to + 1, Order::Desc)
        };

        let mut pages = question::Entity::find()
            .filter(question::Column::Page.eq(page))
            .order_by(question::Column::Id, order)
            .offset(offset as u64)
            .limit(Some(limit as u64))
            .stream(&*DATABASE).await.unwrap();

        let first = pages.next().await.ok_or(ErrorMessage::NotFound)?
            .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;

        let mut current = first.clone();

        while let Some(next) = pages.next().await {
            let next = next.map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;

            let mut active = current.into_active_model();
            active.answer = Set(next.answer.clone());
            active.sub_points = Set(next.sub_points);
            active.all_points = Set(next.all_points);
            active.content = Set(next.content.clone());
            active.condition = Set(next.condition.clone());
            active.required = Set(next.required);
            active.r#type = Set(next.r#type);
            active.values = Set(next.values.clone());
            active.update(&*DATABASE).await.unwrap();

            current = next;
        }

        let mut active = current.into_active_model();
        active.answer = Set(first.answer.clone());
        active.sub_points = Set(first.sub_points);
        active.all_points = Set(first.all_points);
        active.content = Set(first.content.clone());
        active.condition = Set(first.condition.clone());
        active.required = Set(first.required);
        active.r#type = Set(first.r#type);
        active.values = Set(first.values.clone());
        active.update(&*DATABASE).await.unwrap();


        Ok(())
    }

    pub async fn create(new_question: NewQuestion) -> Result<Self, ErrorMessage> {
        let (answer, all_points, sub_points) = if let Some(a) = new_question.answer {
            (Some(a.answer), a.all_points, a.sub_points)
        } else {
            (None, None, None)
        };

        let question = question::ActiveModel {
            id: NotSet,
            page: Set(new_question.page),
            content: Set(serde_json::to_string(&new_question.content).unwrap()),
            r#type: Set(new_question.r#type),
            values: Set(new_question.values.map(|v| serde_json::to_string(&v).unwrap())),
            condition: Set(new_question.condition.map(|c| serde_json::to_string(&c).unwrap())),
            required: Set(new_question.required),
            answer: Set(answer),
            all_points: Set(all_points),
            sub_points: Set(sub_points),
        };

        question.insert(&*crate::DATABASE).await
            .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?.to_modal()
    }

    pub async fn get_access(&self) -> Result<bool, ErrorMessage> {
        let page = page::Model::find_by_id(self.page).await?;
        page.check_access().await.map(|a| a.0)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Question {
    pub id: i32,
    pub content: ValueWithTitle,
    pub page: i32,
    pub r#type: QuestionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<ValueWithTitle>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Vec<Condition>>,
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<Answer>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Condition {
    pub r#type: ConditionType,
    pub conditions: Vec<ConditionInner>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConditionInner {
    pub id: i32,
    pub value: JsonValue,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConditionType {
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
    #[serde(rename = "not")]
    Not,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Answer {
    pub all_points: Option<i32>,
    pub sub_points: Option<i32>,
    pub answer: String,
}

#[derive(Deserialize)]
pub struct NewQuestion {
    pub content: ValueWithTitle,
    pub page: i32,
    pub r#type: QuestionType,
    pub values: Option<Vec<ValueWithTitle>>,
    pub condition: Option<Vec<Condition>>,
    pub required: bool,
    pub answer: Option<Answer>,
}

impl question::Model {
    pub fn to_modal(&self) -> Result<Question, ErrorMessage> {
        Ok(Question {
            id: self.id,
            content: serde_json::from_str(&self.content).unwrap(),
            page: self.page,
            r#type: self.r#type,
            values: self.values.clone().map(|v| serde_json::from_str(&v).map_err(|_| ErrorMessage::InvalidField {
                field: String::from("values"),
                should_be: String::from("json"),
            })).transpose()?,
            condition: self.condition.clone().map(|c| serde_json::from_str(&c).map_err(|_| ErrorMessage::InvalidField {
                field: String::from("condition"),
                should_be: String::from("json"),
            })).transpose()?,
            required: self.required,
            answer: self.answer.clone().map(|ans| Answer {
                all_points: self.all_points,
                sub_points: self.sub_points,
                answer: ans,
            }),
        })
    }
}

impl Question {
    pub fn to_entity(&self) -> question::Model {
        let (answer, all_points, sub_points) = if let Some(a) = &self.answer {
            (Some(a.answer.clone()), a.all_points, a.sub_points)
        } else {
            (None, None, None)
        };

        question::Model {
            id: self.id,
            page: self.page,
            content: serde_json::to_string(&self.content).unwrap(),
            r#type: self.r#type,
            values: self.values.clone().map(|v| serde_json::to_string(&v).unwrap()),
            condition: self.condition.clone().map(|c| serde_json::to_string(&c).unwrap()),
            required: self.required,
            answer,
            all_points,
            sub_points,
        }
    }
}


/*
1 2 3 4 5
  f   t
offset 1
limit 3

1 4 2 3 5
  f   t
offset 1
limit 3
*/
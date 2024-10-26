use crate::controller::error::ErrorMessage;
use crate::dao::entity::question;
use crate::dao::entity::question::QuestionType;
use crate::dao::model::ValueWithTitle;
use lazy_static::lazy_static;
use moka::future::Cache;
use sea_orm::ActiveValue::Set;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, NotSet, QueryOrder};
use sea_orm::{ColumnTrait, JsonValue};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

lazy_static! {
    pub static ref QUESTION_CACHE: Cache<String, question::Model> = Cache::new(10000);
}

impl Question {
    pub async fn find_by_id(id: &str) -> Result<Self, ErrorMessage> {
        if let Some(a) = QUESTION_CACHE.get(id).await {
            return a.to_modal();
        }

        let question = question::Entity::find()
            .filter(question::Column::Id.eq(Uuid::parse_str(id).map_err(|_| ErrorMessage::InvalidField {
                field: String::from("id"),
                should_be: String::from("uuid")
            })?))
            .one(&*crate::DATABASE).await.unwrap()
            .ok_or(ErrorMessage::NotFound)?;

        QUESTION_CACHE.insert(id.to_string(), question.clone()).await;

        question.to_modal()
    }
    
    pub async fn find_by_page(page_id: &str) -> Result<Vec<Self>, ErrorMessage> {
        let questions = question::Entity::find()
            .filter(question::Column::Page.eq(Uuid::parse_str(page_id).map_err(|_| ErrorMessage::InvalidField {
                field: String::from("page_id"),
                should_be: String::from("uuid")
            })?))
            .order_by_asc(question::Column::Order)
            .all(&*crate::DATABASE).await.unwrap();

        questions.iter().map(|q| q.clone().to_modal()).collect::<Result<Vec<_>, _>>()
    }
    
    pub async fn update(&self) {
        QUESTION_CACHE.invalidate(&self.id.to_string()).await;
        
        let entity = self.to_entity().into_active_model();
        entity.update(&*crate::DATABASE).await.unwrap();
    }
    
    pub async fn create(new_question: NewQuestion) -> Option<Self> {
        let (answer, all_points, sub_points) = if let Some(a) = new_question.answer {
            (Some(serde_json::to_string(&a).unwrap()), Some(a.all_points), a.sub_points)
        } else {
            (None, None, None)
        };
        
        let question = question::ActiveModel {
            id: Set(Uuid::new_v4()),
            page: Set(new_question.page),
            order: NotSet,
            content: Set(new_question.content),
            r#type: Set(new_question.r#type),
            values: Set(new_question.values.map(|v| serde_json::to_string(&v).unwrap())),
            condition: Set(new_question.condition.map(|c| serde_json::to_string(&c).unwrap())),
            required: Set(new_question.required),
            answer: Set(answer),
            all_points: Set(all_points),
            sub_points: Set(sub_points),
        };
        
        question.insert(&*crate::DATABASE).await.ok()?.to_modal().ok()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Question {
    pub id: Uuid,
    pub content: String,
    pub page: Uuid,
    pub order: i32,
    pub r#type: QuestionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<ValueWithTitle>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Vec<Condition>>,
    pub required: bool,
    #[serde(skip_serializing)]
    pub answer: Option<Answer>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Condition {
    pub r#type: ConditionType,
    pub conditions: Vec<ConditionInner>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConditionInner {
    pub id: Uuid,
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
    pub all_points: i32,
    pub sub_points: Option<i32>,
    pub answer: String,
}

#[derive(Deserialize)]
pub struct NewQuestion {
    pub content: String,
    pub page: Uuid,
    pub r#type: QuestionType,
    pub values: Option<Vec<ValueWithTitle>>,
    pub condition: Option<Vec<Condition>>,
    pub required: bool,
    pub answer: Option<Answer>,
}

impl question::Model {
    fn to_modal(&self) -> Result<Question, ErrorMessage> {
        
        Ok(Question {
            id: self.id,
            content: self.content.clone(),
            page: self.page,
            order: self.order,
            r#type: self.r#type,
            values: self.values.clone().map(|v| serde_json::from_str(&v).map_err(|_| ErrorMessage::InvalidField {
                field: String::from("values"),
                should_be: String::from("json")
            })).transpose()?,
            condition: self.condition.clone().map(|c| serde_json::from_str(&c).map_err(|_| ErrorMessage::InvalidField {
                field: String::from("condition"),
                should_be: String::from("json")
            })).transpose()?,
            required: self.required,
            answer: self.clone().answer.map(|a| serde_json::from_str(&a).map_err(|_| ErrorMessage::InvalidField {
                field: String::from("answer"),
                should_be: String::from("json")
            })).transpose()?,
        })
    }
}

impl Question {
    pub fn to_entity(&self) -> question::Model {
        let (answer, all_points, sub_points) = if let Some(a) = &self.answer {
            (Some(serde_json::to_string(&a).unwrap()), Some(a.all_points), a.sub_points)
        } else {
            (None, None, None)
        };
        
        question::Model {
            id: self.id,
            page: self.page,
            order: self.order,
            content: self.content.clone(),
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
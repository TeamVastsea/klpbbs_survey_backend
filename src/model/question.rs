use crate::model::generated::question::Model;
use crate::model::ValueWithTitle;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Question {
    pub id: Uuid,
    pub content: ValueWithTitle,
    pub r#type: QuestionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<ValueWithTitle>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Vec<Condition>>,
    pub required: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Condition {
    pub r#type: ConditionType,
    pub conditions: Vec<ConditionInner>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConditionInner {
    pub id: Uuid,
    pub value: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConditionType {
    And,
    Or,
    Not,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum QuestionType {
    Text = 1,
    MultipleChoice = 2,
    SingleChoice = 3,
    File = 4,
}

impl Question {
    pub fn new(question_type: QuestionType, content: ValueWithTitle, values: Option<Vec<ValueWithTitle>>,
               condition: Option<Vec<Condition>>, required: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            content,
            r#type: question_type,
            values,
            condition,
            required,
        }
    }
}

impl TryFrom<u8> for QuestionType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(QuestionType::Text),
            2 => Ok(QuestionType::MultipleChoice),
            3 => Ok(QuestionType::SingleChoice),
            4 => Ok(QuestionType::File),
            _ => Err("Invalid value for QuestionType")
        }
    }
}

impl TryFrom<Model> for Question {

    type Error = String;

    fn try_from(value: Model) -> Result<Self, Self::Error> {
        let content: ValueWithTitle = serde_json::from_value(value.content).unwrap();
        let values: Option<Vec<ValueWithTitle>> = value.values.map(|values| 
            values.iter().map(|v| serde_json::from_value(v.clone()).unwrap()).collect());
        let condition: Option<Vec<Condition>> = value.condition.map(|condition| 
            serde_json::from_str(&condition).unwrap());

        Ok(Question {
            id: Uuid::from_str(&value.id).unwrap(),
            content,
            r#type: QuestionType::try_from(value.r#type as u8)?,
            values,
            condition,
            required: value.required,
        })
    }
}
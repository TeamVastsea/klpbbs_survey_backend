use crate::controller::error::ErrorMessage;
use crate::dao::model::page::PAGE_CACHE;
use crate::dao::model::question::QUESTION_CACHE;
use serde::{Deserialize, Serialize};

pub mod user_data;
pub mod page;
pub mod question;
mod score;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PagedData<T> {
    pub data: Vec<T>,
    pub total: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ValueWithTitle {
    pub title: String,
    pub content: String,
}

impl TryFrom<String> for ValueWithTitle {
    type Error = ErrorMessage;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&value).map_err(|_| ErrorMessage::InvalidField {
            field: String::from("ValueWithTitle"),
            should_be: String::from("json"),
        })
    }
}

pub fn refresh_cache(refresh_type: CacheType) {
    match refresh_type {
        CacheType::Question => {
            QUESTION_CACHE.invalidate_all();
        }
        CacheType::Page => {
            PAGE_CACHE.invalidate_all();
        }
        CacheType::Both => {
            QUESTION_CACHE.invalidate_all();
            PAGE_CACHE.invalidate_all();
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CacheType {
    Question,
    Page,
    Both,
}
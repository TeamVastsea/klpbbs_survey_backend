use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::model::ValueWithTitle;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Page {
    pub id: Uuid,
    pub title: ValueWithTitle,
    pub questions: Vec<Uuid>,
    pub next: Option<Uuid>,
}
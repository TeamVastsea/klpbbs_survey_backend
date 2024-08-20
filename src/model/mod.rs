use serde::{Deserialize, Serialize};

pub mod generated;
pub mod question;
pub mod page;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ValueWithTitle {
    pub title: String,
    pub content: String
}
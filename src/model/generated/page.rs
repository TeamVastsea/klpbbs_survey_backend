//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "page")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub title: String,
    pub content: Vec<String>,
    pub next: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "survey")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub budge: String,
    pub description: String,
    pub image: String,
    pub page: String,
    pub start_date: DateTime,
    pub end_date: DateTime,
    pub allow_submit: bool,
    pub allow_view: bool,
    pub allow_judge: bool,
    pub allow_re_submit: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

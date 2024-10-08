//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "answer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub survey: i32,
    pub user: i64,
    pub answers: Json,
    pub score: Option<i32>,
    pub create_time: DateTime,
    pub completed: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::score::Entity")]
    Score
}

impl Related<super::score::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Score.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

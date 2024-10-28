//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "score")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub survey: i32,
    pub user: String,
    pub answer: String,
    pub completed: bool,
    pub update_time: DateTime,
    pub judge: Option<String>,
    pub judge_time: Option<DateTime>,
    pub scores: Option<String>,
    pub user_scores: Option<i32>,
    pub full_scores: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::survey::Entity",
        from = "Column::Survey",
        to = "super::survey::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Survey,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Judge",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User2,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::User",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User1,
}

impl Related<super::survey::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Survey.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
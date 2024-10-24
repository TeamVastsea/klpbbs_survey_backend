use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{boolean, integer, integer_null, pk_uuid, string, string_null, uuid};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Question::Table)
                    .if_not_exists()
                    .col(pk_uuid(Question::Id))
                    .col(uuid(Question::Page))
                    .col(integer(Question::Order).auto_increment())
                    .col(string(Question::Content))
                    .col(integer(Question::Type))
                    .col(string_null(Question::Values))
                    .col(string_null(Question::Condition))
                    .col(boolean(Question::Required))
                    .col(string_null(Question::Answer))
                    .col(integer_null(Question::AllPoints))
                    .col(integer_null(Question::SubPoints))
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_question_page")
                    .from_tbl(Question::Table)
                    .from_col(Question::Page)
                    .to_tbl(Page::Table)
                    .to_col(Page::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Question::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Question {
    Table,
    Id,
    Page,
    Order,
    Content,
    Type,
    Values,
    Condition,
    Required,
    Answer,
    AllPoints,
    SubPoints,
}

#[derive(DeriveIden)]
enum Page {
    Table,
    Id,
}

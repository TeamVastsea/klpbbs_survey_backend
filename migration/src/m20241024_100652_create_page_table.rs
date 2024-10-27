use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{integer, pk_auto, string};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Page::Table)
                    .if_not_exists()
                    .col(pk_auto(Page::Id))
                    .col(string(Page::Title))
                    .col(integer(Page::Survey))
                    .to_owned(),
            )
            .await?;
        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_page_survey")
                .from_tbl(Page::Table)
                .from_col(Page::Survey)
                .to_tbl(Survey::Table)
                .to_col(Survey::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Page::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Page {
    Table,
    Id,
    Title,
    Survey,
}

#[derive(DeriveIden)]
enum Survey {
    Table,
    Id,
}
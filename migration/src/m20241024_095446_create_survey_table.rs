use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{boolean, char_len, pk_auto, string, timestamp};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Survey::Table)
                    .if_not_exists()
                    .col(pk_auto(Survey::Id))
                    .col(char_len(Survey::Badge, 10))
                    .col(string(Survey::Description))
                    .col(string(Survey::Image))
                    .col(timestamp(Survey::StartDate))
                    .col(timestamp(Survey::EndDate))
                    .col(boolean(Survey::AllowSubmit).default(true))
                    .col(boolean(Survey::AllowView).default(true))
                    .col(boolean(Survey::AllowJudge).default(true))
                    .col(boolean(Survey::AllowReSubmit).default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Survey::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Survey {
    Table,
    Id,
    Badge,
    Description,
    Image,
    StartDate,
    EndDate,
    AllowSubmit,
    AllowView,
    AllowJudge,
    AllowReSubmit,
}
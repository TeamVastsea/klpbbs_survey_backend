use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(Survey::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Survey::Title).string().not_null())
                    .col(ColumnDef::new(Survey::Budge).char_len(10).not_null())
                    .col(ColumnDef::new(Survey::Description).string().not_null())
                    .col(ColumnDef::new(Survey::Image).string().not_null())
                    .col(ColumnDef::new(Survey::StartDate).timestamp().not_null())
                    .col(ColumnDef::new(Survey::EndDate).timestamp().not_null())
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
    Title,
    Budge,
    Description,
    Image,
    StartDate,
    EndDate,
}

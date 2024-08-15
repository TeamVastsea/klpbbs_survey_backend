use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(Question::Id)
                            .char_len(36)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Question::Content).json().not_null())
                    .col(ColumnDef::new(Question::Type).integer().not_null())
                    .col(ColumnDef::new(Question::Values).array(ColumnType::Json).null())
                    .col(ColumnDef::new(Question::Condition).text().null())
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
    Content,
    Type,
    Values,
    Condition,
}

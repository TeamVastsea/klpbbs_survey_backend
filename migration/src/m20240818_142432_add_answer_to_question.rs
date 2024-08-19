use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Question::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Question::Answer)
                            .string()
                            .null()
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Question::AllPoints)
                            .unsigned()
                            .null()
                            .default(2)
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Question::SubPoints)
                            .unsigned()
                            .null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Question {
    Table,
    Answer,
    AllPoints,
    SubPoints,
}

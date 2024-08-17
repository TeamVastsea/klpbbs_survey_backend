use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Survey::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Survey::AllowSubmit)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Survey::AllowView)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Survey::AllowJudge)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Survey::AllowReSubmit)
                            .boolean()
                            .not_null()
                            .default(false),
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
enum Survey {
    Table,
    AllowSubmit,
    AllowView,
    AllowJudge,
    AllowReSubmit,
}

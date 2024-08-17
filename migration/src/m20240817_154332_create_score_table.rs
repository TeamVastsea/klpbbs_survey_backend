use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Score::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Score::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Score::User).big_unsigned().not_null())
                    .col(ColumnDef::new(Score::Judge).big_unsigned().not_null())
                    .col(ColumnDef::new(Score::Survey).integer().not_null())
                    .col(ColumnDef::new(Score::Answer).integer().not_null())
                    .col(ColumnDef::new(Score::Scores).array(ColumnType::Json).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Score::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Score {
    Table,
    Id,
    User,
    Judge,
    Survey,
    Answer,
    Scores,
}

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
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Score::User).big_unsigned().not_null())
                    .col(ColumnDef::new(Score::Judge).big_unsigned().not_null())
                    .col(ColumnDef::new(Score::JudgeTime).timestamp().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(Score::Scores).json().not_null())
                    .col(ColumnDef::new(Score::UserScore).integer().not_null())
                    .col(ColumnDef::new(Score::FullScore).integer().not_null())
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
    JudgeTime,
    Scores,
    UserScore,
    FullScore,
}

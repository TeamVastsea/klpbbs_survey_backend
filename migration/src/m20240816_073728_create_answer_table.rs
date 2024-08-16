use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Answer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Answer::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Answer::Survey).string().not_null())
                    .col(ColumnDef::new(Answer::User).big_unsigned().not_null())
                    .col(ColumnDef::new(Answer::Judge).big_unsigned().null())
                    .col(ColumnDef::new(Answer::Answers).json().not_null())
                    .col(ColumnDef::new(Answer::Score).integer().null())
                    .col(ColumnDef::new(Answer::CreateTime).timestamp().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Answer::JudgedTime).timestamp().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Answer::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Answer {
    Table,
    Id,
    Survey,
    User,
    Judge,
    Answers,
    Score,
    CreateTime,
    JudgedTime,
}

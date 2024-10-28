use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{boolean, integer, integer_null, pk_auto, string, string_null, timestamp, timestamp_null};

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
                    .col(pk_auto(Score::Id))
                    .col(integer(Score::Survey))
                    .col(string(Score::User))
                    .col(string(Score::Answer))
                    .col(boolean(Score::Completed).default(false))
                    .col(timestamp(Score::UpdateTime))
                    .col(string_null(Score::Judge))
                    .col(timestamp_null(Score::JudgeTime))
                    .col(string_null(Score::Scores))
                    .col(integer_null(Score::UserScores))
                    .col(integer_null(Score::FullScores))
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_score_survey")
                    .from_tbl(Score::Table)
                    .from_col(Score::Survey)
                    .to_tbl(Survey::Table)
                    .to_col(Survey::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_score_user")
                    .from_tbl(Score::Table)
                    .from_col(Score::User)
                    .to_tbl(User::Table)
                    .to_col(User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_score_judge")
                    .from_tbl(Score::Table)
                    .from_col(Score::Judge)
                    .to_tbl(User::Table)
                    .to_col(User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
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
    Survey,
    User,
    Answer,
    Completed,
    UpdateTime,
    Judge,
    JudgeTime,
    Scores,
    UserScores,
    FullScores,
}

#[derive(DeriveIden)]
enum Survey {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}

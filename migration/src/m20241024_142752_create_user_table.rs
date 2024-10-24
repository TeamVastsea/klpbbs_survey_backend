use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{boolean, char_len, char_len_null, string, string_null, string_uniq, timestamp, timestamp_null};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(string(User::Id).primary_key())
                    .col(string_null(User::Credential).unique_key())
                    .col(boolean(User::Admin).default(false))
                    .col(boolean(User::Disabled).default(false))
                    .col(string(User::Username))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Credential,
    IssuedAt,
    Admin,
    Disabled,
    Username,
}

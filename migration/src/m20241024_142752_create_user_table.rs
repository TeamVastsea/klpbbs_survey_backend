use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{boolean, string, string_null};

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
            .await?;

        manager.exec_stmt(
            Query::insert()
                .into_table(User::Table)
                .columns(vec![User::Id, User::Username, User::Admin])
                .values_panic([
                    "admin".into(),
                    "admin".into(),
                    true.into(),
                ])
                .to_owned()
        ).await
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
    Admin,
    Disabled,
    Username,
}

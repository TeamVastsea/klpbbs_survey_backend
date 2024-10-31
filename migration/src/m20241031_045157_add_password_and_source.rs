use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{integer, integer_null, string_null};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column_if_not_exists(string_null(User::Password))
                    .add_column_if_not_exists(integer(User::UserSource).default(0))
                    .to_owned()
            )
            .await?;
        manager.exec_stmt(
            Query::update()
                .table(User::Table)
                .and_where(Expr::col(User::Username).eq("admin"))
                .values(vec![
                    (User::Password, "ExthaUwUxBZd7rAa$Smphx226TIwgrp3bntPAawKsxJxz+L3AHp0aM5uQaFw=".into()),
                    (User::UserSource, 1.into()),
                ])
                .to_owned()
        ).await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Survey::Table)
                    .add_column_if_not_exists(integer_null(Survey::UserSource))
                    .to_owned()
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Username,
    Password,
    UserSource,
}

#[derive(DeriveIden)]
enum Survey {
    Table,
    UserSource,
}
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Page::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Page::Id)
                            .char_len(36)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Page::Title).string().not_null())
                    .col(ColumnDef::new(Page::Content).array(ColumnType::Char(Some(36))).not_null())
                    .col(ColumnDef::new(Page::Next).integer().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Page::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Page {
    Table,
    Id,
    Title,
    Content,
    Next,
}

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Accounts::Table)
                    .if_not_exists()
                    .col(pk_auto(Accounts::Id))
                    .col(string(Accounts::Username).not_null().unique_key())
                    .col(string(Accounts::Name).not_null())
                    .col(string(Accounts::Email).not_null())
                    .col(string(Accounts::PasswordHash).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Accounts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Accounts {
    Table,
    Id,
    Username,
    Name,
    Email,
    PasswordHash,
}

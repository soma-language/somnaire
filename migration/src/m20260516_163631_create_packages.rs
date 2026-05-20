use crate::m20260516_163507_create_users::Accounts;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Packages::Table)
                    .if_not_exists()
                    .col(pk_auto(Packages::Id))
                    .col(string(Packages::Name).not_null())
                    .col(string(Packages::Version).not_null())
                    .col(string(Packages::Author).not_null())
                    .col(string(Packages::License).not_null())
                    .col(string(Packages::Repository).not_null())
                    .col(string(Packages::Description).not_null())
                    .col(integer(Packages::Downloads).not_null().default(0))
                    .col(date_time(Packages::CreatedAt).not_null())
                    .col(date_time(Packages::UpdatedAt).not_null())
                    .col(integer(Packages::CreatedBy).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-package-created_by-account-id")
                            .from(Packages::Table, Packages::CreatedBy)
                            .to(Accounts::Table, Accounts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Packages::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Packages {
    Table,
    Id,
    Name,
    Version,
    Author,
    License,
    Repository,
    Description,
    Downloads,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
}

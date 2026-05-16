use sea_orm_migration::{prelude::*, schema::*};
use crate::m20260516_163507_create_users::Account;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Package::Table)
                    .if_not_exists()
                    .col(pk_auto(Package::Id))
                    .col(string(Package::Name).not_null())
                    .col(string(Package::Version).not_null())
                    .col(string(Package::Author).not_null())
                    .col(string(Package::License).not_null())
                    .col(string(Package::Repository).not_null())
                    .col(string(Package::Description).not_null())
                    .col(date_time(Package::CreatedAt).not_null())
                    .col(date_time(Package::UpdatedAt).not_null())
                    .col(integer(Package::CreatedBy).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-package-created_by-account-id")
                            .from(Package::Table, Package::CreatedBy)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Package::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Package {
    Table,
    Id,
    Name,
    Version,
    Author,
    License,
    Repository,
    Description,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
}

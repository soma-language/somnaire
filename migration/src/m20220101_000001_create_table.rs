use sea_orm_migration::{prelude::*, schema::*};

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
}

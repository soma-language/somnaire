#![allow(clippy::wildcard_imports)]

pub use sea_orm_migration::prelude::*;

mod m20260516_163507_create_users;
mod m20260516_163631_create_packages;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260516_163507_create_users::Migration),
            Box::new(m20260516_163631_create_packages::Migration),
        ]
    }
}

pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20241024_224838_create_vaults_database;
mod m20241024_230139_create_vault_entry_table;
mod m20241025_001045_link_uer_and_vault;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20241024_224838_create_vaults_database::Migration),
            Box::new(m20241024_230139_create_vault_entry_table::Migration),
            Box::new(m20241025_001045_link_uer_and_vault::Migration),
        ]
    }
}

use sea_orm_migration::prelude::*;

use crate::m20241024_230139_create_vault_entry_table::VaultEntry;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .rename_table(
                Table::rename()
                    .table(VaultEntry::Table, Alias::new("vault_entries"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .rename_table(
                Table::rename()
                    .table(VaultEntry::Table, Alias::new("vault_entry"))
                    .to_owned(),
            )
            .await
    }
}

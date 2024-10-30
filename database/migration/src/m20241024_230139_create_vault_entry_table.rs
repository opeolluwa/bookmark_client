use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241024_224838_create_vaults_database::Vault;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(VaultEntry::Table)
                    .if_not_exists()
                    .col(uuid(VaultEntry::Id).unique_key().primary_key())
                    .col(string(VaultEntry::Title).not_null())
                    .col(string(VaultEntry::Description))
                    .col(uuid(VaultEntry::VaultId).unique_key())
                    .col(
                        timestamp_with_time_zone(VaultEntry::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(VaultEntry::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-vault-to-vault-entry-id")
                            .from(VaultEntry::Table, VaultEntry::VaultId)
                            .to(Vault::Table, Vault::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(VaultEntry::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum VaultEntry {
    Table,
    Id,
    Title,
    Description,
    VaultId,
    CreatedAt,
    UpdatedAt,
}

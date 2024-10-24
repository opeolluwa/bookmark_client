use sea_orm_migration::{prelude::*, schema::*};

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
                    .col(string(VaultEntry::Title))
                    .col(string(VaultEntry::Description))
                    .col(string(VaultEntry::DateAdded))
                    .col(string(VaultEntry::LastModified))
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
enum VaultEntry {
    Table,
    Id,
    Title,
    Description,
    DateAdded,
    // Keywords,
    LastModified
}

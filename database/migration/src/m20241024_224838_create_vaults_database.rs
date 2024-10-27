use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Vault::Table)
                    .if_not_exists()
                    .col(uuid(Vault::Id).unique_key().primary_key())
                    .col(string(Vault::Name))
                    .col(string(Vault::Description))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Vault::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Vault {
    Table,
    Id,
    Name,
    Description,
}

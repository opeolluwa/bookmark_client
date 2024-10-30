use super::m20220101_000001_create_table::UserInformation;
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
                    .col(uuid(Vault::Id).unique_key().primary_key().not_null())
                    .col(string(Vault::Name).not_null())
                    .col(string(Vault::Description))
                    .col(uuid(Vault::UserId).not_null())
                    .col(
                        timestamp_with_time_zone(Vault::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Vault::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-vault-id")
                            .from(Vault::Table, Vault::UserId)
                            .to(UserInformation::Table, UserInformation::Id),
                    )
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
pub enum Vault {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
    UserId,
}

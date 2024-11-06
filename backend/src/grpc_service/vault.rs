use std::str::FromStr;

use entities::prelude::VaultEntries;
use entities::vault;
use log::trace;
use prost_types::Timestamp;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use tonic::async_trait;
use uuid::Uuid;

use crate::{
    database_connection::DatabaseConnection,
    proto::vault::{
        vault_manager_server::VaultManager, DeleteVaultRequest, DeleteVaultResponse,
        GetVaultRequest, GetVaultResponse, ListVaultsRequest, ListVaultsResponse, NewVaultRequest,
        NewVaultResponse, UpdateVaultRequest, UpdateVaultResponse,
    },
};

#[derive(Default)]
pub struct VaultManagerImplementation {}

#[async_trait]
impl VaultManager for VaultManagerImplementation {
    async fn create_new_vault(
        &self,
        request: tonic::Request<NewVaultRequest>,
    ) -> std::result::Result<tonic::Response<NewVaultResponse>, tonic::Status> {
        let (metadata, _, payload) = request.into_parts();
        let db_connection = &DatabaseConnection::new().await;

        let Some(user_id) = metadata.get("user_id") else {
            return Err(tonic::Status::unauthenticated(
                "Missing or badly formatted authorization header",
            ));
        };

        let user_id = user_id.to_str().map_err(|_| {
            tonic::Status::not_found("Missing or badly formatted authorization header")
        })?;

        let vault_id = Uuid::new_v4();
        let new_vault = vault::ActiveModel {
            id: Set(vault_id),
            name: Set(payload.name.trim().to_string()),
            description: Set(payload.description.trim().to_string()),
            user_id: Set(Uuid::from_str(user_id).unwrap()),
            ..Default::default()
        };

        let db_insert_result = vault::Entity::insert(new_vault)
            .exec(db_connection)
            .await
            .map_err(|err| {
                tonic::Status::unknown(format!(
                    "The server couldn't process the request at this time due to err {}",
                    err.to_string()
                ))
            })?;

        let Some(created_vault) = vault::Entity::find_by_id(db_insert_result.last_insert_id)
            .one(db_connection)
            .await
            .map_err(|err| {
                trace!("{:#?}", err.to_string());
                tonic::Status::unknown(format!(
                    "The server couldn't process the request at this time sue to err {}",
                    err.to_string()
                ))
            })?
        else {
            return Err(tonic::Status::unknown(
                "The server couldn't process the request at this time sue to err",
            ));
        };

        let message = NewVaultResponse {
            vault_id: db_insert_result.last_insert_id.into(),
            user_id: user_id.into(),
            name: created_vault.name.clone(),
            description: created_vault.description.clone(),
            updated_at: Some(
                Timestamp::from_str(&created_vault.created_at.to_string())
                    .expect("Failed to parse updated_at timestamp"),
            ),
            created_at: Some(
                Timestamp::from_str(&created_vault.created_at.to_string())
                    .expect("Failed to parse created_at timestamp"),
            ),
        };

        Ok(tonic::Response::new(message))
    }
    async fn get_vault(
        &self,
        request: tonic::Request<GetVaultRequest>,
    ) -> std::result::Result<tonic::Response<GetVaultResponse>, tonic::Status> {
        let (metadata, _, payload) = request.into_parts();
        let db_connection = &DatabaseConnection::new().await;

        let Some(user_id) = metadata.get("user_id") else {
            return Err(tonic::Status::unauthenticated(
                "Missing or badly formatted authorization header",
            ));
        };

        let user_id = user_id.to_str().map_err(|_| {
            tonic::Status::not_found("Missing or badly formatted authorization header")
        })?;

        let vault_id = Uuid::from_str(&payload.vault_id).unwrap();

        let Some(record) = vault::Entity::find_by_id(vault_id)
            .find_also_related(VaultEntries)
            .one(db_connection)
            .await
            .map_err(|err| {
                trace!("{:#?}", err.to_string());
                tonic::Status::unknown(format!(
                    "The server couldn't process the request at this time sue to err {}",
                    err.to_string()
                ))
            })?
        else {
            return Err(tonic::Status::not_found(
                "No record with the provided was found",
            ));
        };

        let vault: entities::vault::Model = record.0;
        // let entries = record.1.expect("no entries was found");

        let message = GetVaultResponse {
            vault_id: vault_id.into(),
            name: vault.name,
            description: vault.description,
            updated_at: Some(Timestamp::from_str(&vault.created_at.to_string()).unwrap()),
            created_at: Some(Timestamp::from_str(&vault.created_at.to_string()).unwrap()),
            user_id: user_id.into(),
            entries: [].to_vec(), //TODO: fix this
        };

        Ok(tonic::Response::new(message))
    }
    async fn update_vault(
        &self,
        request: tonic::Request<UpdateVaultRequest>,
    ) -> std::result::Result<tonic::Response<UpdateVaultResponse>, tonic::Status> {
        todo!()
    }
    async fn delete_vault(
        &self,
        request: tonic::Request<DeleteVaultRequest>,
    ) -> std::result::Result<tonic::Response<DeleteVaultResponse>, tonic::Status> {
        todo!()
    }
    async fn list_vaults(
        &self,
        request: tonic::Request<ListVaultsRequest>,
    ) -> std::result::Result<tonic::Response<ListVaultsResponse>, tonic::Status> {
        let (metadata, _, _payload) = request.into_parts();
        let db_connection = &DatabaseConnection::new().await;

        let Some(user_id) = metadata.get("user_id") else {
            return Err(tonic::Status::unauthenticated(
                "Missing or badly formatted authorization header",
            ));
        };

        let user_id = user_id.to_str().map_err(|_| {
            tonic::Status::not_found("Missing or badly formatted authorization header")
        })?;

        let records = vault::Entity::find()
            .filter(entities::vault::Column::UserId.eq::<Uuid>(Uuid::from_str(&user_id).unwrap()))
            .order_by_asc(entities::vault::Column::CreatedAt)
            .all(db_connection)
            .await
            .map_err(|err| {
                trace!("{:#?}", err.to_string());
                tonic::Status::unknown(format!(
                    "The server couldn't process the request at this time sue to err {}",
                    err.to_string()
                ))
            })?;

        let vaults = records
            .into_iter()
            .map(|record| GetVaultResponse {
                vault_id: record.id.into(),
                name: record.name,
                description: record.description,
                updated_at: Some(Timestamp::from_str(&record.created_at.to_string()).unwrap()),
                created_at: Some(Timestamp::from_str(&record.created_at.to_string()).unwrap()),
                user_id: user_id.into(),
                entries: [].to_vec(), //TODO: fix this
            })
            .collect::<Vec<crate::proto::vault::GetVaultResponse>>();

        let message = ListVaultsResponse {
            vaults,
            total_count: 5i32,
        };

        Ok(tonic::Response::new(message))
    }
}

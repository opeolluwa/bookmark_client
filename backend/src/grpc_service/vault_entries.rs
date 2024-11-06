use tonic::async_trait;

use crate::proto::vault_entries::{
    vault_entries_manager_server::VaultEntriesManager, DeleteVaultEntryResponse,
    GetVaultEntryRequest, GetVaultEntryResponse, ListVaultEntryRequest, ListVaultEntryResponse,
    NewVaultEntryRequest, UpdateVaultEntryRequest,
};

#[derive(Default)]
pub struct VaultEntriesManagerImplementation {}

#[async_trait]
impl VaultEntriesManager for VaultEntriesManagerImplementation {
    async fn create_new_vault(
        &self,
        request: tonic::Request<NewVaultEntryRequest>,
    ) -> std::result::Result<tonic::Response<GetVaultEntryResponse>, tonic::Status> {
        todo!()
    }
    async fn get_vault_entry(
        &self,
        request: tonic::Request<GetVaultEntryRequest>,
    ) -> std::result::Result<tonic::Response<GetVaultEntryResponse>, tonic::Status> {
        todo!()
    }
    async fn update_vault_entry(
        &self,
        request: tonic::Request<UpdateVaultEntryRequest>,
    ) -> std::result::Result<tonic::Response<GetVaultEntryResponse>, tonic::Status> {
        todo!()
    }
    async fn delete_vault_entry(
        &self,
        request: tonic::Request<UpdateVaultEntryRequest>,
    ) -> std::result::Result<tonic::Response<DeleteVaultEntryResponse>, tonic::Status> {
        todo!()
    }
    async fn list_vault_entries(
        &self,
        request: tonic::Request<ListVaultEntryRequest>,
    ) -> std::result::Result<tonic::Response<ListVaultEntryResponse>, tonic::Status> {
        todo!()
    }
}

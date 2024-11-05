use tonic::async_trait;

use crate::proto::vault::{vault_manager_server::VaultManager, DeleteVaultRequest, DeleteVaultResponse, GetVaultRequest, GetVaultResponse, ListVaultsRequest, ListVaultsResponse, NewVaultRequest, NewVaultResponse, UpdateVaultRequest, UpdateVaultResponse};


#[derive(Default)]
pub struct VaultManagerImplementation {}


#[async_trait]
impl VaultManager for VaultManagerImplementation {
  async fn create_new_vault(
            &self,
            request: tonic::Request<NewVaultRequest>,
        ) -> std::result::Result<
            tonic::Response<NewVaultResponse>,
            tonic::Status,
        >{
            todo!()
        }
        async fn get_vault(
            &self,
            request: tonic::Request<GetVaultRequest>,
        ) -> std::result::Result<
            tonic::Response<GetVaultResponse>,
            tonic::Status,
        >{
            todo!()
        }
        async fn update_vault(
            &self,
            request: tonic::Request<UpdateVaultRequest>,
        ) -> std::result::Result<
            tonic::Response<UpdateVaultResponse>,
            tonic::Status,
        >{
            todo!()
        }
        async fn delete_vault(
            &self,
            request: tonic::Request<DeleteVaultRequest>,
        ) -> std::result::Result<
            tonic::Response<DeleteVaultResponse>,
            tonic::Status,
        >{
            todo!()
        }
        async fn list_vaults(
            &self,
            request: tonic::Request<ListVaultsRequest>,
        ) -> std::result::Result<
            tonic::Response<ListVaultsResponse>,
            tonic::Status,
        > {
            todo!()
        }
}
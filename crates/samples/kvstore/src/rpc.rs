#![allow(dead_code)] // TODO: fix

use mssf_core::client::FabricClient;
use tonic::async_trait;

tonic::include_proto!("kvstore_rpc"); // The string specified here must match the proto package name

struct KvRpc {
    fc: FabricClient,
}

#[async_trait]
impl kvstore_service_server::KvstoreService for KvRpc {
    async fn list_db(
        &self,
        _request: tonic::Request<EmptyPayload>,
    ) -> std::result::Result<tonic::Response<ListDbResponse>, tonic::Status> {
        // list the services in the app
        todo!()
    }
}

use std::sync::Arc;

use mssf_ext::state_replicator::StateReplicatorProxy;
use tonic::async_trait;

use crate::app::KvApp;

tonic::include_proto!("kvmap_rpc"); // The string specified here must match the proto package name

pub struct KvMapRpc {
    app: Arc<KvApp>,
    sr: StateReplicatorProxy,
}

impl KvMapRpc {
    pub fn new(app: Arc<KvApp>, sr: StateReplicatorProxy) -> Self {
        Self { app, sr }
    }
}

#[async_trait]
impl kvmap_service_server::KvmapService for KvMapRpc {
    async fn get_data(
        &self,
        _request: tonic::Request<EmptyPayload>,
    ) -> std::result::Result<tonic::Response<DataSnPayload>, tonic::Status> {
        let (sn, data) = self.app.get_data().await?;
        let res = DataSnPayload { data, sn };
        Ok(tonic::Response::new(res))
    }

    async fn set_data(
        &self,
        request: tonic::Request<DataSnPayload>,
    ) -> std::result::Result<tonic::Response<StatusPayload>, tonic::Status> {
        let payload = request.into_inner();
        let data = payload.data;
        let sn = self.app.set_data_client(&self.sr, data).await?;
        let res = StatusPayload { ok: true, sn };
        Ok(tonic::Response::new(res))
    }
}

use bytes::Buf;
use mssf_com::{
    FabricRuntime::IFabricStateProvider,
    FabricTypes::{FABRIC_REPLICATOR_ADDRESS, FABRIC_REPLICATOR_SETTINGS},
};
use mssf_core::{
    runtime::{
        executor::{DefaultExecutor, Executor},
        stateful::{
            PrimaryReplicator, StatefulServiceFactory, StatefulServicePartition,
            StatefulServiceReplica,
        },
        stateful_proxy::PrimaryReplicatorProxy,
        stateful_types::{Epoch, OpenMode, Role},
    },
    GUID, HSTRING,
};
use mssf_ext::{
    state_provider::StateProviderBridge,
    traits::{OperationDataStream, StateProvider},
};
use tracing::{error, info};
use windows_core::Interface;

use crate::data::CountingOperationDataStream;

pub struct Factory {
    replication_port: u32,
    rt: DefaultExecutor,
}

impl Factory {
    pub fn create(replication_port: u32, rt: DefaultExecutor) -> Self {
        Self {
            replication_port,
            rt,
        }
    }
}

impl StatefulServiceFactory for Factory {
    fn create_replica(
        &self,
        servicetypename: &mssf_core::HSTRING,
        servicename: &mssf_core::HSTRING,
        initializationdata: &[u8],
        partitionid: &GUID,
        replicaid: i64,
    ) -> mssf_core::Result<impl StatefulServiceReplica> {
        info!("create_replica: service_type {servicetypename}, service_name {servicename}, init data len {}, partition id {:?}, replicaid {replicaid}",
          initializationdata.len(),
          partitionid
        );
        Ok(Replica::create(self.replication_port, self.rt.clone()))
    }
}

pub struct Replica {
    replication_port: u32,
    rt: DefaultExecutor,
}

impl Replica {
    fn create(replication_port: u32, rt: DefaultExecutor) -> Self {
        Self {
            replication_port,
            rt,
        }
    }
}

impl StatefulServiceReplica for Replica {
    async fn open(
        &self,
        _openmode: OpenMode,
        partition: &StatefulServicePartition,
    ) -> mssf_core::Result<impl PrimaryReplicator> {
        let com = partition.get_com();

        let stateprovider = KvStateProvider::create(self.rt.clone());
        let stateprovider_bridge: IFabricStateProvider =
            StateProviderBridge::new(stateprovider, self.rt.clone()).into();

        let addr = HSTRING::from(format!("{}:{}", "localhost", self.replication_port));
        let settings = FABRIC_REPLICATOR_SETTINGS {
            Flags: FABRIC_REPLICATOR_ADDRESS.0 as u32,
            ReplicatorAddress: mssf_core::PCWSTR(addr.as_ptr()),
            ..Default::default()
        };

        // return addr for replicator
        let mut rplctr = None;
        let res = unsafe {
            com.CreateReplicator(
                &stateprovider_bridge,
                &settings,
                std::ptr::addr_of_mut!(rplctr),
            )
        };
        if res.is_err() {
            error!("CreateReplicator failed: {:?}", res);
            return Err(res.err().unwrap());
        }
        // return the replicator.
        let rplctr = rplctr.unwrap().cast().unwrap();
        let proxy = PrimaryReplicatorProxy::new(rplctr);
        Ok(proxy)
    }

    async fn change_role(&self, _newrole: Role) -> mssf_core::Result<HSTRING> {
        Ok(HSTRING::from("myhost:12345"))
    }

    async fn close(&self) -> mssf_core::Result<()> {
        Ok(())
    }

    fn abort(&self) {}
}

pub struct KvStateProvider {
    rt: DefaultExecutor,
}

impl KvStateProvider {
    fn create(rt: DefaultExecutor) -> Self {
        Self { rt }
    }
}

impl StateProvider for KvStateProvider {
    async fn update_epoch(
        &self,
        epoch: &Epoch,
        previousepochlastsequencenumber: i64,
    ) -> mssf_core::Result<()> {
        info!(
            "update_epoch: Epoch: {:?}, previous epoch lsn: {}",
            epoch, previousepochlastsequencenumber
        );
        Ok(())
    }

    fn get_last_committed_sequence_number(&self) -> mssf_core::Result<i64> {
        info!("get_last_committed_sequence_number: 0");
        Ok(0)
    }
    async fn on_data_loss(&self) -> mssf_core::Result<bool> {
        info!("on_data_loss");
        Ok(false)
    }
    fn get_copy_context(&self) -> mssf_core::Result<impl OperationDataStream> {
        Ok(CountingOperationDataStream::new(3, "copycontext"))
    }
    fn get_copy_state(
        &self,
        upto_sequence_number: i64,
        copy_context_stream: impl OperationDataStream,
    ) -> mssf_core::Result<impl OperationDataStream> {
        // copy context should be from secondary
        // read the context

        self.rt.spawn(async move {
            // read the context stream.
            while let Some(mut c) = copy_context_stream.get_next().await.unwrap() {
                let b = c.copy_to_bytes(c.remaining());
                let s = String::from_utf8_lossy(&b).into_owned();
                info!("get_copy_state: copy_context_stream: {}", s);
            }
        });
        // returned stream should be copy to secondary
        Ok(CountingOperationDataStream::new(
            upto_sequence_number as usize,
            "copystate",
        ))
    }
}

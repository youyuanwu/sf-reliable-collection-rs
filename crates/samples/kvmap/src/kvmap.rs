use std::{cell::Cell, sync::Arc};

use bytes::{Buf, Bytes};
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
    data::OperationDataBuf,
    state_provider::StateProviderBridge,
    state_replicator::StateReplicatorProxy,
    traits::{
        LocalOperationStream, LocalStateReplicator, Operation, OperationDataStream, StateProvider,
    },
};
use tokio::{select, sync::Mutex};
use tokio_util::sync::CancellationToken;
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

#[derive(Clone)]
pub struct ReplicaState {
    lsn: Arc<std::sync::Mutex<Cell<i64>>>,
}

impl ReplicaState {
    fn create() -> Self {
        Self {
            lsn: Arc::new(std::sync::Mutex::new(Cell::new(0))),
        }
    }

    // add 1 and return the result
    // async fn increment(&self) -> i64 {
    //     let mut lsn = self.lsn.lock().await;
    //     *lsn.get_mut() += 1;
    //     lsn.get()
    // }

    // set the value and return the prev
    fn apply(&self, lsn: i64) -> i64 {
        let mut lk = self.lsn.lock().unwrap();
        let prev = lk.get();
        *lk.get_mut() = lsn;
        prev
    }

    // async fn get(&self) -> i64 {
    //     let lsn = self.lsn.lock().await;
    //     lsn.get()
    // }

    fn get_sync(&self) -> i64 {
        let lk = self.lsn.lock().unwrap();
        lk.get()
    }
}

pub struct Replica {
    replication_port: u32,
    rt: DefaultExecutor,
    state_replicator: Mutex<Cell<Option<StateReplicatorProxy>>>,
    cancel: Mutex<Cell<Option<CancellationToken>>>,
    state: ReplicaState,
    role: Mutex<Cell<Role>>,
}

impl Replica {
    fn create(replication_port: u32, rt: DefaultExecutor) -> Self {
        Self {
            replication_port,
            rt,
            state_replicator: Mutex::new(Cell::new(None)),
            cancel: Mutex::new(Cell::new(None)),
            state: ReplicaState::create(),
            role: Mutex::new(Cell::new(Role::Unknown)),
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

        let stateprovider = KvStateProvider::create(self.rt.clone(), self.state.clone());
        let stateprovider_bridge: IFabricStateProvider =
            StateProviderBridge::new(stateprovider, self.rt.clone()).into();

        let mut rplctr;
        let state_rplctr;
        {
            let addr = HSTRING::from(format!("{}:{}", "localhost", self.replication_port));
            let settings = FABRIC_REPLICATOR_SETTINGS {
                Flags: FABRIC_REPLICATOR_ADDRESS.0 as u32,
                ReplicatorAddress: mssf_core::PCWSTR(addr.as_ptr()),
                ..Default::default()
            };

            // return addr for replicator
            rplctr = None;
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
            state_rplctr = res.unwrap();
        }
        // save the state replicator in self
        let sr = self.state_replicator.lock().await;
        let prev = sr.replace(Some(StateReplicatorProxy::new(
            state_rplctr.cast().unwrap(),
        )));
        assert!(prev.is_none());

        // initiate the cancel token for closing.
        //let token = CancellationToken::new();
        let token = self.cancel.lock().await;
        let prev = token.replace(Some(CancellationToken::new()));
        assert!(prev.is_none());

        // return the replicator.
        let rplctr = rplctr.unwrap().cast().unwrap();
        let proxy = PrimaryReplicatorProxy::new(rplctr);
        Ok(proxy)
    }

    async fn change_role(&self, newrole: Role) -> mssf_core::Result<HSTRING> {
        // get the state replicator opened.
        let sr = self
            .state_replicator
            .lock()
            .await
            .get_mut()
            .clone()
            .unwrap();
        let dummy_addr = HSTRING::from("myhost:12345");
        // clean up pending stuff
        let curr_role = self.role.lock().await.get_mut().clone();
        if curr_role == newrole {
            // nothing has changed.
            return Ok(dummy_addr);
        }

        if !matches!(curr_role, Role::None | Role::Unknown) {
            // has background stuff running, cancel it
            let mut lk = self.cancel.lock().await;
            // cancel the prev token and init a new one
            let prev = lk.get_mut().replace(CancellationToken::new());
            prev.unwrap().cancel();
        };

        let token = self.cancel.lock().await.get_mut().clone().unwrap();
        let state = self.state.clone();
        match newrole {
            Role::ActiveSecondary => {
                // Handle replicate stream from primary
                self.rt.spawn(async move {
                    let rplct_stream = sr.get_replication_stream().unwrap();
                    loop {
                        let opt = select! {
                            _ = token.cancelled() => { None }
                            res = rplct_stream.get_operation() => {
                                res.unwrap() // get op should be ok
                            }
                        };

                        if opt.is_none() {
                            info!("replication stream end or cancelled.");
                            break;
                        }
                        // apply the data with lsn
                        let op = opt.unwrap();
                        let lsn = op.get_metadate().sequence_number;
                        let prev_lsn = state.apply(lsn);
                        let b = op.get_data().unwrap();
                        let s = String::from_utf8_lossy(b.chunk()).into_owned();
                        op.acknowledge().unwrap();
                        info!("rplct_stream: {}, lsn: {}, prev: {}", s, lsn, prev_lsn);
                    }
                });
            }
            Role::IdleSecondary => {
                self.rt.spawn(async move {
                    // handle copying. i.e. catch up from primary. The stream is from copy_state from primary
                    let copy_stream = sr.get_copy_stream().unwrap();
                    while let Some(c) = copy_stream.get_operation().await.unwrap() {
                        let b = c.get_data().unwrap();
                        let s = String::from_utf8_lossy(b.chunk()).into_owned();
                        info!("KvStateProvider::change_role: copy_stream: {}", s);
                        // ack the data is applied
                        c.acknowledge().unwrap();
                    }
                    info!("KvStateProvider: Completed copy stream catchup on idle secondary.")
                })
            }
            Role::None => {
                // delete stuff on disk?
            }
            Role::Primary => {
                // start replicate?
                self.rt.spawn(async move {
                    let mut i = 0;
                    loop {
                        select! {
                            _ = token.cancelled() => {
                                info!("replicate loop cancelled.");
                                break ;
                            }
                            _ = tokio::time::sleep(std::time::Duration::from_secs(15)) => {}
                        };
                        let mut out = 0_i64;
                        let data =
                            OperationDataBuf::new(Bytes::from(format!("replicate-data-{i}")));
                        let out2 = sr.replicate(data, &mut out).await.unwrap();
                        assert_eq!(out, out2);
                        let prev = state.apply(out);
                        info!("replicated lsn {out}, prev {prev}");
                        i += 1;
                    }
                })
            }
            Role::Unknown => panic!("Unknonw role"),
        }

        Ok(dummy_addr)
    }

    async fn close(&self) -> mssf_core::Result<()> {
        let token = self.cancel.lock().await.get_mut().clone();
        if let Some(t) = token {
            t.cancel();
        }
        Ok(())
    }

    fn abort(&self) {
        // cancel but sync. This is in SF thread pool.
        let token = self.cancel.blocking_lock().get_mut().clone();
        if let Some(t) = token {
            t.cancel();
        }
    }
}

pub struct KvStateProvider {
    rt: DefaultExecutor,
    state: ReplicaState,
}

impl KvStateProvider {
    fn create(rt: DefaultExecutor, state: ReplicaState) -> Self {
        Self { rt, state }
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
        let lsn = self.state.get_sync();
        info!("get_last_committed_sequence_number: {lsn}");
        Ok(lsn)
    }
    async fn on_data_loss(&self) -> mssf_core::Result<bool> {
        info!("on_data_loss");
        Ok(false)
    }

    // invoked on secondary.
    fn get_copy_context(&self) -> mssf_core::Result<impl OperationDataStream> {
        info!("KvStateProvider::get_copy_state: get_copy_context");
        Ok(CountingOperationDataStream::new(3, "copycontext"))
    }

    // Invoked on primary. Data stream from above get_copy_context from secondary is presented here.
    fn get_copy_state(
        &self,
        upto_sequence_number: i64,
        copy_context_stream: impl OperationDataStream,
    ) -> mssf_core::Result<impl OperationDataStream> {
        // copy context should be from secondary
        // read the context in background
        self.rt.spawn(async move {
            // read the context stream.
            while let Some(mut c) = copy_context_stream.get_next().await.unwrap() {
                let b = c.copy_to_bytes(c.remaining());
                let s = String::from_utf8_lossy(&b).into_owned();
                info!(
                    "KvStateProvider::get_copy_state: copy_context_stream: {}",
                    s
                );
            }
        });
        info!(
            "KvStateProvider::get_copy_state: lsn upto {upto_sequence_number}, current lsn {}",
            self.state.get_sync()
        );
        // returned stream should be copy to secondary
        Ok(CountingOperationDataStream::new(
            upto_sequence_number as usize,
            "copystate",
        ))
    }
}

use std::{
    cell::Cell,
    ffi::c_void,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;

use fabric_c::Microsoft::ServiceFabric::ReliableCollectionRuntime::{
    IFabricDataLossHandler, TxnReplicator_Settings,
};
use fabric_rs::runtime::{
    executor::{DefaultExecutor, Executor},
    stateful::{
        PrimaryReplicator, StatefulServiceFactory, StatefulServicePartition, StatefulServiceReplica,
    },
    stateful_proxy::PrimaryReplicatorProxy,
    stateful_types::{OpenMode, Role},
};
use log::info;
use reliable_collection::wrap::{get_txn_replicator, TxnReplicaReplicator};
use tokio::{
    select,
    sync::oneshot::{self, Sender},
};
use windows_core::{Error, HSTRING};

use crate::utils::DataLossHandler;

pub struct Factory {
    replication_port: u32,
    rt: DefaultExecutor,
}

impl Factory {
    pub fn create(replication_port: u32, rt: DefaultExecutor) -> Factory {
        Factory {
            replication_port,
            rt,
        }
    }
}

fn get_addr(port: u32, hostname: HSTRING) -> String {
    let mut addr = String::new();
    addr.push_str(&hostname.to_string());
    addr.push(':');
    addr.push_str(&port.to_string());
    addr
}

impl StatefulServiceFactory<Replica> for Factory {
    fn create_replica(
        &self,
        servicetypename: &windows_core::HSTRING,
        servicename: &windows_core::HSTRING,
        initializationdata: &[u8],
        partitionid: &windows::core::GUID,
        replicaid: i64,
    ) -> Result<Replica, Error> {
        info!(
            "Factory::create_replica type {}, service {}, init data size {}, partid {:?}",
            servicetypename,
            servicename,
            initializationdata.len(),
            partitionid
        );

        let svc = Service::new(self.rt.clone());

        let replica = Replica::new(replicaid, svc, self.replication_port);
        Ok(replica)
    }
}

pub struct Replica {
    svc: Service,
    id: i64,
    port: u32,
}

impl Replica {
    pub fn new(id: i64, svc: Service, port: u32) -> Replica {
        Replica { svc, id, port }
    }
}

// The serving of the database.
pub struct Service {
    rt: DefaultExecutor,
    store: Mutex<Cell<Option<Arc<TxnReplicaReplicator>>>>,
    tx: Mutex<Cell<Option<Sender<()>>>>,
}

impl Service {
    pub fn new(rt: DefaultExecutor) -> Service {
        Service {
            rt,
            store: Mutex::new(Cell::new(None)),
            tx: Mutex::new(Cell::new(None)),
        }
    }

    pub fn set_store(&self, store: TxnReplicaReplicator) {
        let prev = self.store.lock().unwrap().replace(Some(Arc::new(store)));
        assert!(prev.is_none());
    }

    fn get_store(&self) -> Arc<TxnReplicaReplicator> {
        self.store.lock().unwrap().get_mut().clone().unwrap()
    }

    pub fn start_loop(&self) {
        let (tx, mut rx) = oneshot::channel::<()>();
        self.stop();
        self.tx.lock().unwrap().set(Some(tx));
        let store = self.get_store();
        self.rt.spawn(async move {
            let mut counter = 0;
            loop {
                info!("Service::run_single: {}", counter);
                Self::run_single(store.clone()).await.unwrap();

                counter += 1;
                // sleep or stop
                select! {
                    _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
                        continue;
                    }
                    _ = &mut rx =>{
                        info!("Service::loop stopped from rx");
                        break;
                    }
                }
            }
        });
    }

    pub fn stop(&self) {
        let mut op = self.tx.lock().unwrap().take();
        if op.is_some() {
            op.take().unwrap().send(()).unwrap()
        }
    }

    async fn run_single(kv: Arc<TxnReplicaReplicator>) -> windows_core::Result<()> {
        let info = kv.txn_replicator_get_info();
        match info {
            Ok(t_info) => {
                info!("replicator info {:?}", t_info)
            }
            Err(e) => {
                info!("replicator info err {}", e)
            }
        }
        Ok(())
    }
}

#[async_trait]
impl StatefulServiceReplica for Replica {
    async fn open(
        &self,
        openmode: OpenMode,
        partition: &StatefulServicePartition,
    ) -> windows::core::Result<Box<dyn PrimaryReplicator>> {
        // should be primary replicator
        info!("Replica::open {:?}", openmode);

        let dataloss_handler: IFabricDataLossHandler = DataLossHandler {}.into();

        let addr = get_addr(self.port, HSTRING::from("localhost"));
        let waddr = HSTRING::from(addr);

        let txn_settings = TxnReplicator_Settings {
            ReplicatorAddress: waddr.as_ptr() as *mut c_void,
            ..Default::default()
        };
        // create reliable collection.
        let ok = get_txn_replicator(
            self.id,
            partition.get_com(),
            &dataloss_handler,
            &txn_settings,
            &HSTRING::new(),
            &HSTRING::new(),
            &HSTRING::new(),
        );
        if ok.is_err() {
            let e = ok.err().unwrap();
            info!("get_txn_replicator failed with {}", e);
            return Err(e);
        }

        let (p, txnp) = ok.unwrap();

        self.svc.set_store(txnp);

        let p_proxy = Box::new(PrimaryReplicatorProxy::new(p));
        Ok(p_proxy)
    }
    async fn change_role(&self, newrole: Role) -> ::windows_core::Result<HSTRING> {
        info!("Replica::change_role {:?}", newrole);
        //let addr = self.kv.change_role(newrole.clone()).await?;
        if newrole == Role::Primary {
            self.svc.start_loop();
        }
        let addr = HSTRING::from("my addr");
        Ok(addr)
    }
    async fn close(&self) -> windows::core::Result<()> {
        info!("Replica::close");
        self.svc.stop();
        Ok(())
    }
    fn abort(&self) {
        info!("Replica::abort");
        self.svc.stop();
    }
}

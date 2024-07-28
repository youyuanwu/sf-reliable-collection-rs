use std::time::Duration;

use mssf_core::{
    client::{
        query_client::QueryClient,
        svc_mgmt_client::{PartitionKeyType, ServiceEndpointRole, ServiceManagementClient},
        FabricClient,
    },
    types::{
        ServicePartition, ServicePartitionInformation, ServicePartitionQueryDescription,
        ServicePartitionStatus,
    },
    GUID, HSTRING,
};
use tokio::sync::Semaphore;

use crate::rpc::{DataSnPayload, EmptyPayload};
use lazy_static::lazy_static;

// limit 1 test at a time.
static PERMIT: Semaphore = Semaphore::const_new(1);
static TIMEOUT: Duration = Duration::from_secs(2);
lazy_static! {
    static ref KV_MAP_SVC_URI: HSTRING = HSTRING::from("fabric:/KvMap/KvMapService");
    static ref FABRIC_CLIENT: FabricClient = FabricClient::new();
}
// helper for managing app
pub struct KvMapMgmt {
    svc: ServiceManagementClient,
    query: QueryClient,
}

impl KvMapMgmt {
    pub fn new(c: &FabricClient) -> Self {
        Self {
            svc: c.get_service_manager(),
            query: c.get_query_manager(),
        }
    }

    // first is primary
    pub async fn get_addrs(&self) -> (String, String) {
        let resolution = self
            .svc
            .resolve_service_partition(
                &KV_MAP_SVC_URI,
                &PartitionKeyType::None,
                None,
                Duration::from_secs(1),
            )
            .await
            .unwrap();
        // find endpoints
        let endpoints = resolution.get_endpoint_list();

        // there is only 2 replicas

        let primary_addr = endpoints
            .iter()
            .find(|e| e.role == ServiceEndpointRole::StatefulPrimary)
            .expect("no primary found")
            .address
            .to_string();
        let secondary_addr = endpoints
            .iter()
            .find(|e| e.role == ServiceEndpointRole::StatefulSecondary)
            .expect("no secondary found")
            .address
            .to_string();
        (primary_addr, secondary_addr)
    }

    pub async fn get_partition(&self) -> (GUID, ServicePartitionStatus) {
        let desc = ServicePartitionQueryDescription {
            service_name: KV_MAP_SVC_URI.clone(),
            partition_id_filter: None,
        };
        let list = self.query.get_partition_list(&desc, TIMEOUT).await.unwrap();
        // there is only one partition
        let p = list.iter().next().unwrap();
        let stateful = match p {
            ServicePartition::Stateful(s) => s,
            _ => panic!("not stateless"),
        };
        let info = stateful.partition_information;
        let single = match info {
            ServicePartitionInformation::Singleton(s) => s,
            _ => panic!("not singleton"),
        };
        (single.id, stateful.partition_status)
    }
}

#[tokio::test]
async fn read_write_test() {
    let _ = PERMIT.acquire().await.unwrap();

    // resolve port on local onebox
    let c = KvMapMgmt::new(&FABRIC_CLIENT);
    let (primary_addr, secondary_addr) = c.get_addrs().await;

    println!("primary_addr: {}", primary_addr);
    // connect primary via grpc
    let mut client = crate::rpc::kvmap_service_client::KvmapServiceClient::connect(primary_addr)
        .await
        .unwrap();
    // connect secondary
    println!("secondary_addr: {}", secondary_addr);
    let mut sec_client =
        crate::rpc::kvmap_service_client::KvmapServiceClient::connect(secondary_addr)
            .await
            .unwrap();

    // set data and read
    {
        let data = "mydata";
        // sn is ignored for now
        let req = tonic::Request::new(DataSnPayload {
            data: data.to_string(),
            sn: -1,
        });
        let response = client.set_data(req).await.expect("rpc failed").into_inner();
        let sn = response.sn;
        assert!(response.ok);
        assert_ne!(sn, 0);
        println!("RESPONSE={:?}", response);

        // read from primary
        {
            let req2 = tonic::Request::new(EmptyPayload {});
            let response2 = client.get_data(req2).await.expect("rpc faile").into_inner();
            assert_eq!(response2.data, data);
            assert_eq!(sn, response2.sn);
            println!("RESPONSE={:?}", response2);
        }
        // read from secondary
        {
            let req2 = tonic::Request::new(EmptyPayload {});
            let response2 = sec_client
                .get_data(req2)
                .await
                .expect("rpc faile")
                .into_inner();
            assert_eq!(response2.data, data);
            assert_eq!(sn, response2.sn);
            println!("RESPONSE={:?}", response2);
        }
    }
}

// TODO: perform failover.
#[tokio::test]
async fn failover_test() {
    let _ = PERMIT.acquire().await.unwrap();
    let c = KvMapMgmt::new(&FABRIC_CLIENT);
    let (_, status) = c.get_partition().await;
    assert_eq!(status, ServicePartitionStatus::Ready)
}

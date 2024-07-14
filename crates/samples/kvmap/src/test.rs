use std::time::Duration;

use mssf_core::{
    client::{
        svc_mgmt_client::{PartitionKeyType, ServiceEndpointRole},
        FabricClient,
    },
    HSTRING,
};

use crate::rpc::{DataSnPayload, EmptyPayload};

#[tokio::test]
async fn read_write_test() {
    // resolve port on local onebox
    let fc = FabricClient::new();
    let svcc = fc.get_service_manager();
    let resolution = svcc
        .resolve_service_partition(
            &HSTRING::from("fabric:/KvMap/KvMapService"),
            &PartitionKeyType::None,
            None,
            Duration::from_secs(1),
        )
        .await
        .unwrap();
    // find endpoints
    let endpoints = resolution.get_endpoint_list();

    let primary_addr = endpoints
        .iter()
        .find(|e| e.role == ServiceEndpointRole::StatefulPrimary)
        .expect("no primary found")
        .address
        .to_string();
    println!("primary_addr: {}", primary_addr);
    // connect primary via grpc
    let mut client = crate::rpc::kvmap_service_client::KvmapServiceClient::connect(primary_addr)
        .await
        .unwrap();

    // connect secondary
    let secondary_addr = endpoints
        .iter()
        .find(|e| e.role == ServiceEndpointRole::StatefulSecondary)
        .expect("no secondary found")
        .address
        .to_string();
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
          let response2 = sec_client.get_data(req2).await.expect("rpc faile").into_inner();
          assert_eq!(response2.data, data);
          assert_eq!(sn, response2.sn);
          println!("RESPONSE={:?}", response2);
        }
    }
}

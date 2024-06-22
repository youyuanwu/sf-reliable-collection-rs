use std::time::Duration;

use clap::Parser;
use mssf_core::{
    client::{
        svc_mgmt_client::{PartitionKeyType, ServiceEndpointRole},
        FabricClient,
    },
    HSTRING,
};

tonic::include_proto!("kvstore_rpc"); // The string specified here must match the proto package name

use crate::kvstore_service_client::KvstoreServiceClient;

#[derive(Parser)] // requires `derive` feature
#[command(name = "kvcli")]
#[command(bin_name = "kvcli")]
enum KvCli {
    List(ListArgs),
    Get(GetArgs),
    Add(AddArgs),
    Remove(RemoveArgs),
}

#[derive(clap::Args)]
#[command(version, about, long_about = None)]
struct ListArgs {
    #[arg(long)]
    db: String,
}

#[derive(clap::Args)]
#[command(version, about, long_about = None)]
struct GetArgs {
    #[arg(long)]
    db: String,
    #[arg(long)]
    key: String,
}

#[derive(clap::Args)]
#[command(version, about, long_about = None)]
struct AddArgs {
    #[arg(long)]
    db: String,
    #[arg(long)]
    key: String,
    #[arg(long)]
    val: String,
}

#[derive(clap::Args)]
#[command(version, about, long_about = None)]
struct RemoveArgs {
    #[arg(long)]
    db: String,
    #[arg(long)]
    key: String,
}

#[tokio::main]
async fn main() {
    let cli = KvCli::parse();

    // resolve port on local onebox
    let fc = FabricClient::new();
    let svcc = fc.get_service_manager();
    let resolution = svcc
        .resolve_service_partition(
            &HSTRING::from("fabric:/KvStore/KvStoreService"),
            &PartitionKeyType::None,
            None,
            Duration::from_secs(1),
        )
        .await
        .unwrap();
    // find primary
    let endpoint = resolution
        .get_endpoint_list()
        .iter()
        .find(|e| e.role == ServiceEndpointRole::StatefulPrimary)
        .expect("no primary found");
    let addr = endpoint.address.to_string();

    println!("Using kvstore addr: {}", addr);
    let mut client = KvstoreServiceClient::connect(addr).await.unwrap();

    match cli {
        KvCli::List(args) => {
            let store_url = format!("fabric:/{}", args.db);
            let req = tonic::Request::new(EnumerateRequest {
                store_url: store_url.clone(),
            });
            let resp = client.enumerate_all(req).await.expect("cannot list");
            println!("List={:?}", resp.into_inner().payload);
        }
        KvCli::Get(args) => {
            let store_url = format!("fabric:/{}", args.db);
            let req = tonic::Request::new(GetRequest {
                store_url,
                key: args.key,
            });
            let response = client.get(req).await.expect("cannot get kv");
            println!("Get={:?}", response.into_inner());
        }
        KvCli::Add(args) => {
            let store_url = format!("fabric:/{}", args.db);
            let req = tonic::Request::new(AddRequest {
                store_url,
                key: args.key,
                val: args.val,
            });
            let response = client.add(req).await.expect("cannot add kv");
            println!("Add={:?}", response.into_inner());
        }
        KvCli::Remove(args) => {
            let store_url = format!("fabric:/{}", args.db);
            let req = tonic::Request::new(RemoveRequest {
                store_url,
                key: args.key,
                conditional_version: -1, // -1 means ignore
            });
            let response = client.remove(req).await.expect("cannot remove kv");
            println!("Remove={:?}", response.into_inner());
        }
    }
}
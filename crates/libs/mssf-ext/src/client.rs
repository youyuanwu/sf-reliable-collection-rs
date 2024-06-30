#![allow(dead_code)] // TODO fix

use mssf_com::{
    FabricClient::{IFabricGetServiceListResult, IFabricQueryClient10},
    FabricTypes::FABRIC_SERVICE_QUERY_DESCRIPTION,
};
use mssf_core::sync::{self, FabricReceiver};

pub struct QueryClientExt(IFabricQueryClient10);

impl QueryClientExt {
    pub fn create() -> Self {
        Self(sync::CreateLocalClient::<IFabricQueryClient10>())
    }

    fn get_service_list_internal(
        &self,
        desc: &FABRIC_SERVICE_QUERY_DESCRIPTION,
        timeout_milliseconds: u32,
    ) -> FabricReceiver<mssf_core::Result<IFabricGetServiceListResult>> {
        let (tx, rx) = sync::oneshot_channel();
        let com_cp = self.0.clone();
        let callback = sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { com_cp.EndGetServiceList(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.0
                .BeginGetServiceList(desc, timeout_milliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }

    pub fn get_service_list(&self) {}
}

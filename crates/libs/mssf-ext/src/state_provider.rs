use std::sync::Arc;

use mssf_com::{
    FabricCommon::{IFabricAsyncOperationCallback, IFabricAsyncOperationContext},
    FabricRuntime::{IFabricOperationDataStream, IFabricStateProvider, IFabricStateProvider_Impl},
    FabricTypes::FABRIC_EPOCH,
};
use mssf_core::{
    runtime::{executor::Executor, stateful_types::Epoch},
    sync::{fabric_begin_bridge, fabric_end_bridge},
};
use tracing::info;
use windows_core::implement;

use crate::{
    stream::{OpeartionDataStreamBridge, OperationDataStreamProxy},
    traits::StateProvider,
};

// given a state provider trait,
// wrap it to be IFabricStateProvider
#[implement(IFabricStateProvider)]
pub struct StateProviderBridge<T, E>
where
    T: StateProvider,
    E: Executor,
{
    inner: Arc<T>,
    rt: E,
}

impl<T, E> StateProviderBridge<T, E>
where
    T: StateProvider,
    E: Executor,
{
    pub fn new(inner: T, rt: E) -> Self {
        Self {
            inner: Arc::new(inner),
            rt,
        }
    }
}

impl<T: StateProvider, E: Executor> IFabricStateProvider_Impl for StateProviderBridge<T, E> {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn BeginUpdateEpoch(
        &self,
        epoch: *const FABRIC_EPOCH,
        previousepochlastsequencenumber: i64,
        callback: Option<&IFabricAsyncOperationCallback>,
    ) -> windows_core::Result<IFabricAsyncOperationContext> {
        info!("StateProviderBridge::BeginUpdateEpoch");
        let epoch2 = Epoch::from(unsafe { epoch.as_ref().unwrap() });
        let inner = self.inner.clone();
        fabric_begin_bridge(&self.rt, callback, async move {
            inner
                .update_epoch(&epoch2, previousepochlastsequencenumber)
                .await
        })
    }

    fn EndUpdateEpoch(
        &self,
        context: Option<&IFabricAsyncOperationContext>,
    ) -> windows_core::Result<()> {
        info!("StateProviderBridge::EndUpdateEpoch");
        fabric_end_bridge(context)
    }

    fn GetLastCommittedSequenceNumber(&self) -> windows_core::Result<i64> {
        self.inner.get_last_committed_sequence_number()
    }

    fn BeginOnDataLoss(
        &self,
        callback: Option<&IFabricAsyncOperationCallback>,
    ) -> windows_core::Result<IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        fabric_begin_bridge(
            &self.rt,
            callback,
            async move { inner.on_data_loss().await },
        )
    }

    fn EndOnDataLoss(
        &self,
        context: Option<&IFabricAsyncOperationContext>,
    ) -> windows_core::Result<u8> {
        fabric_end_bridge(context)
    }

    fn GetCopyContext(&self) -> windows_core::Result<IFabricOperationDataStream> {
        let stream = self.inner.get_copy_context()?;
        let bridge = OpeartionDataStreamBridge::new(stream, self.rt.clone()).into();
        Ok(bridge)
    }

    fn GetCopyState(
        &self,
        uptosequencenumber: i64,
        copycontextstream: Option<&IFabricOperationDataStream>,
    ) -> windows_core::Result<IFabricOperationDataStream> {
        let proxy = OperationDataStreamProxy::new(copycontextstream.unwrap().clone());
        let stream = self.inner.get_copy_state(uptosequencenumber, proxy)?;

        let bridge = OpeartionDataStreamBridge::new(stream, self.rt.clone()).into();
        Ok(bridge)
    }
}

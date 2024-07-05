use std::sync::Arc;

use mssf_com::{
    FabricCommon::{
        IFabricAsyncOperationCallback, IFabricAsyncOperationContext,
        IFabricAsyncOperationContext_Impl,
    },
    FabricRuntime::{IFabricOperationDataStream, IFabricStateProvider, IFabricStateProvider_Impl},
    FabricTypes::FABRIC_EPOCH,
};
use mssf_core::runtime::{bridge::BridgeContext, executor::Executor, stateful_types::Epoch};
use tracing::info;
use windows_core::{implement, AsImpl};

use crate::traits::StateProvider;

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

impl<T: StateProvider, E: Executor> IFabricStateProvider_Impl for StateProviderBridge<T, E> {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn BeginUpdateEpoch(
        &self,
        epoch: *const FABRIC_EPOCH,
        previousepochlastsequencenumber: i64,
        callback: Option<&IFabricAsyncOperationCallback>,
    ) -> windows_core::Result<IFabricAsyncOperationContext> {
        info!("StateProviderBridge::BeginUpdateEpoch");
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<mssf_core::Result<()>>::new(callback_cp).into();

        let epoch2 = Epoch::from(unsafe { epoch.as_ref().unwrap() });

        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp
                .update_epoch(&epoch2, previousepochlastsequencenumber)
                .await;
            let ctx_bridge: &BridgeContext<mssf_core::Result<()>> = unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndUpdateEpoch(
        &self,
        context: Option<&IFabricAsyncOperationContext>,
    ) -> windows_core::Result<()> {
        info!("StateProviderBridge::EndUpdateEpoch");
        let ctx_bridge: &BridgeContext<mssf_core::Result<()>> =
            unsafe { context.unwrap().as_impl() };
        ctx_bridge.consume_content()?;
        Ok(())
    }

    fn GetLastCommittedSequenceNumber(&self) -> windows_core::Result<i64> {
        todo!()
    }

    fn BeginOnDataLoss(
        &self,
        _callback: Option<&IFabricAsyncOperationCallback>,
    ) -> windows_core::Result<IFabricAsyncOperationContext> {
        todo!()
    }

    fn EndOnDataLoss(
        &self,
        _context: Option<&IFabricAsyncOperationContext>,
    ) -> windows_core::Result<u8> {
        todo!()
    }

    fn GetCopyContext(&self) -> windows_core::Result<IFabricOperationDataStream> {
        let _stream = self.inner.get_copy_context()?;
        todo!()
    }

    fn GetCopyState(
        &self,
        _uptosequencenumber: i64,
        _copycontextstream: Option<&IFabricOperationDataStream>,
    ) -> windows_core::Result<IFabricOperationDataStream> {
        // let stream = self.inner.get_copy_state(uptosequencenumber, &None);
        todo!()
    }
}

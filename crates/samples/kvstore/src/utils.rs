// ------------------------------------------------------------
// Copyright 2024 Youyuan Wu
// Licensed under the MIT License (MIT). See License in the repo root for
// license information.
// ------------------------------------------------------------

use mssf_com::FabricCommon::{IFabricAsyncOperationCallback, IFabricAsyncOperationContext};
use mssf_core::AsyncContext;
use sfrc_c::Microsoft::ServiceFabric::ReliableCollectionRuntime::{
    IFabricDataLossHandler, IFabricDataLossHandler_Impl,
};
use windows::core::implement;
use windows_core::Interface;

// dummy handler
#[derive(Debug)]
#[implement(IFabricDataLossHandler)]
pub struct DataLossHandler {}

impl IFabricDataLossHandler_Impl for DataLossHandler {
    fn BeginOnDataLoss(
        &self,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::Result<()> {
        let callback =
            unsafe { IFabricAsyncOperationCallback::from_raw_borrowed(&callback) }.unwrap();
        let ctx: IFabricAsyncOperationContext = AsyncContext::new(Some(callback)).into();
        // TODO: maybe ctx return needs to set first
        unsafe { ctx.Callback().expect("cannot get callback").Invoke(&ctx) };
        unsafe { *context = ctx.into_raw() };
        Ok(())
    }

    fn EndOnDataLoss(
        &self,
        _context: *mut ::core::ffi::c_void,
        isstatechanged: *mut u8,
    ) -> ::windows_core::Result<()> {
        unsafe { *isstatechanged = 0 };
        Ok(())
    }
}

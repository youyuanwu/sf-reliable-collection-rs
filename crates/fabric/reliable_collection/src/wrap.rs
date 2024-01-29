use std::ffi::c_void;

use fabric_base::FabricCommon::FabricRuntime::{
    IFabricPrimaryReplicator, IFabricStatefulServicePartition,
};
use fabric_c::Microsoft::ServiceFabric::ReliableCollectionRuntime::{
    fnNotifyGetOrAddStateProviderAsyncCompletion, CancellationTokenSource_Cancel,
    CancellationTokenSource_Release, GetTxnReplicator, IFabricDataLossHandler,
    ReliableCollectionRuntime_Initialize, ReliableCollectionRuntime_Initialize2,
    StateProvider_Info, Store_Release, TxnReplicator_CreateTransaction, TxnReplicator_GetInfo,
    TxnReplicator_GetOrAddStateProviderAsync, TxnReplicator_Info, TxnReplicator_Release,
    TxnReplicator_Settings, RELIABLECOLLECTION_API_VERSION,
};
use tokio::sync::oneshot::{self, Receiver, Sender};
use windows::{
    core::{Error, Interface, HRESULT, HSTRING, PCWSTR},
    Win32::Foundation::BOOL,
};

// do module init
pub struct ReliableCollectionRuntime {}

impl ReliableCollectionRuntime {
    pub fn create() -> ReliableCollectionRuntime {
        let ok =
            unsafe { ReliableCollectionRuntime_Initialize(RELIABLECOLLECTION_API_VERSION as u16) };
        ok.expect("cannot init");
        ReliableCollectionRuntime {}
    }

    pub fn create_standalone() -> ReliableCollectionRuntime {
        let ok = unsafe {
            ReliableCollectionRuntime_Initialize2(
                RELIABLECOLLECTION_API_VERSION.try_into().unwrap(),
                true,
            )
        };
        ok.expect("cannot init standalone");
        ReliableCollectionRuntime {}
    }
}

// proxy for the txn replicator handle
pub struct TxnReplicaReplicator {
    h: *const std::ffi::c_void,
}

unsafe impl Send for TxnReplicaReplicator {}
unsafe impl Sync for TxnReplicaReplicator {}

pub struct Txn {
    h: *const std::ffi::c_void,
}

impl TxnReplicaReplicator {
    pub fn txn_replicator_get_info(&self) -> Result<TxnReplicator_Info, Error> {
        let mut info = TxnReplicator_Info {
            ..Default::default()
        };
        unsafe { TxnReplicator_GetInfo(self.h, std::ptr::addr_of_mut!(info))? };
        Ok(info)
    }

    pub fn create_transaction(&self) -> Result<Txn, Error> {
        let mut tx_handle = std::ptr::null_mut::<c_void>();
        unsafe { TxnReplicator_CreateTransaction(self.h, std::ptr::addr_of_mut!(tx_handle))? };
        assert!(!tx_handle.is_null());
        Ok(Txn { h: tx_handle })
    }

    pub fn release(self) {
        unsafe { TxnReplicator_Release(self.h) }
    }

    unsafe extern "system" fn get_or_add_state_provider_callback(
        ctx: *mut ::core::ffi::c_void,
        status: ::windows::core::HRESULT,
        // already got from front end
        _store: *mut ::core::ffi::c_void,
        _exist: ::windows::Win32::Foundation::BOOL,
    ) {
        let ctx_back = unsafe { Box::from_raw(ctx as *mut Sender<HRESULT>) };
        // send always success because receiver is present
        let ok = ctx_back.send(status);
        if ok.is_err() {
            // frontend cancelled.
            debug_assert!(false, "frontend cancelled");
        }
    }

    // TxnReplicator_GetOrAddStateProviderAsync
    pub fn get_or_add_state_provider_async(
        &self,
        txn: &Txn,
        name: &HSTRING,
        lang: &HSTRING,
        stateproviderinfo: &StateProvider_Info,
        timeout: i64,
    ) -> Result<(Receiver<HRESULT>, CancellationToken, StateProvider, BOOL), Error>
// async error, already exist
    {
        // cancellation
        let mut cts = std::ptr::null_mut::<c_void>();
        let mut stateprovider = std::ptr::null_mut::<c_void>();
        let mut alreadyexists = BOOL::default();
        let callback: fnNotifyGetOrAddStateProviderAsyncCompletion =
            Some(Self::get_or_add_state_provider_callback);
        let mut synchronouscomplete = BOOL::default();

        let (tx, rx) = oneshot::channel();

        // prepare ctx
        let ctx = Box::new(tx);
        let ctx_raw = &*ctx as *const Sender<HRESULT>;
        std::mem::forget(ctx); // forget in front end.

        unsafe {
            TxnReplicator_GetOrAddStateProviderAsync(
                self.h,
                txn.h,
                PCWSTR(name.as_ptr()),
                PCWSTR(lang.as_ptr()),
                stateproviderinfo as *const StateProvider_Info,
                timeout,
                std::ptr::addr_of_mut!(cts),
                std::ptr::addr_of_mut!(stateprovider),
                std::ptr::addr_of_mut!(alreadyexists),
                callback,
                ctx_raw as *const c_void,
                std::ptr::addr_of_mut!(synchronouscomplete),
            )?
        };

        if synchronouscomplete.as_bool() {
            // ctx is not used by backend
            let ctx_back = unsafe { Box::from_raw(ctx_raw as *mut Sender<HRESULT>) };
            // send always success because receiver is present
            ctx_back.send(HRESULT::default()).unwrap();
        }

        let cancel = CancellationToken { h: cts };
        let store = StateProvider { h: stateprovider };

        Ok((rx, cancel, store, alreadyexists))
    }
}

pub fn get_txn_replicator(
    replicaid: i64,
    partition: &IFabricStatefulServicePartition,
    dataloss_handler: &IFabricDataLossHandler,
    replicator_settings: &TxnReplicator_Settings,
    config_package_name: &HSTRING,
    replicator_settings_section_name: &HSTRING,
    replicator_security_section_name: &HSTRING,
) -> Result<(IFabricPrimaryReplicator, TxnReplicaReplicator), Error> {
    let mut replicator_raw: *mut std::ffi::c_void = std::ptr::null_mut();
    let mut txn_replicator_raw: *mut std::ffi::c_void = std::ptr::null_mut();
    unsafe {
        GetTxnReplicator(
            replicaid,
            partition.as_raw(),
            dataloss_handler.as_raw(),
            replicator_settings,
            PCWSTR(config_package_name.as_ptr()),
            PCWSTR(replicator_settings_section_name.as_ptr()),
            PCWSTR(replicator_security_section_name.as_ptr()),
            std::ptr::addr_of_mut!(replicator_raw),
            std::ptr::addr_of_mut!(txn_replicator_raw),
        )?
    };
    assert!(!replicator_raw.is_null());
    assert!(!txn_replicator_raw.is_null());

    let replicator = unsafe { IFabricPrimaryReplicator::from_raw(replicator_raw) };
    let txn_replicator = TxnReplicaReplicator {
        h: txn_replicator_raw,
    };
    Ok((replicator, txn_replicator))
}

pub struct CancellationToken {
    h: *mut std::ffi::c_void,
}

impl CancellationToken {
    pub fn cancel(&self) {
        unsafe { CancellationTokenSource_Cancel(self.h) };
    }

    // should only be called once.
    fn release(&self) {
        unsafe { CancellationTokenSource_Release(self.h) };
    }
}

impl Drop for CancellationToken {
    fn drop(&mut self) {
        if !self.h.is_null() {
            self.release();
            self.h = std::ptr::null_mut();
        }
    }
}

pub struct StateProvider {
    h: *mut std::ffi::c_void,
}

impl StateProvider {
    // should only be called once.
    // auto drop calls this
    pub fn release(&mut self) {
        assert!(!self.h.is_null());
        unsafe { Store_Release(self.h) };
        self.h = std::ptr::null_mut();
    }
}

impl Drop for StateProvider {
    fn drop(&mut self) {
        if !self.h.is_null() {
            self.release()
        }
    }
}

#[cfg(test)]
mod test {
    use super::ReliableCollectionRuntime;

    #[test]
    fn test_linking() {
        let _runtime = ReliableCollectionRuntime::create();
    }
}

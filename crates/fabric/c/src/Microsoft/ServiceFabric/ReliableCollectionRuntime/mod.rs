#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn Buffer_Release(handle: *mut ::core::ffi::c_void) {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Buffer_Release(handle: *mut ::core::ffi::c_void) -> ();
    }
    Buffer_Release(handle)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn CancellationTokenSource_Cancel(cts: *mut ::core::ffi::c_void) {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn CancellationTokenSource_Cancel(cts: *mut ::core::ffi::c_void) -> ();
    }
    CancellationTokenSource_Cancel(cts)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn CancellationTokenSource_Release(cts: *mut ::core::ffi::c_void) {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn CancellationTokenSource_Release(cts: *mut ::core::ffi::c_void) -> ();
    }
    CancellationTokenSource_Release(cts)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConcurrentQueue_EnqueueAsync(
    concurrentqueue: *const ::core::ffi::c_void,
    txn: *const ::core::ffi::c_void,
    objecthandle: usize,
    bytes: *const ::core::ffi::c_void,
    byteslength: u32,
    timeout: i64,
    cts: *mut *mut ::core::ffi::c_void,
    callback: fnNotifyAsyncCompletion,
    ctx: *const ::core::ffi::c_void,
    synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn ConcurrentQueue_EnqueueAsync(
            concurrentqueue: *const ::core::ffi::c_void,
            txn: *const ::core::ffi::c_void,
            objecthandle: usize,
            bytes: *const ::core::ffi::c_void,
            byteslength: u32,
            timeout: i64,
            cts: *mut *mut ::core::ffi::c_void,
            callback: fnNotifyAsyncCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    ConcurrentQueue_EnqueueAsync(
        concurrentqueue,
        txn,
        objecthandle,
        bytes,
        byteslength,
        timeout,
        cts,
        callback,
        ctx,
        synchronouscomplete,
    )
    .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn ConcurrentQueue_GetCount(
    concurrentqueue: *const ::core::ffi::c_void,
) -> ::windows_core::Result<i64> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn ConcurrentQueue_GetCount(
            concurrentqueue: *const ::core::ffi::c_void,
            count: *mut i64,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    ConcurrentQueue_GetCount(concurrentqueue, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConcurrentQueue_TryDequeueAsync(
    concurrentqueue: *const ::core::ffi::c_void,
    txn: *const ::core::ffi::c_void,
    timeout: i64,
    objecthandle: *mut usize,
    value: *mut Buffer,
    cts: *mut *mut ::core::ffi::c_void,
    succeeded: *mut ::windows::Win32::Foundation::BOOL,
    callback: fnNotifyTryDequeueAsyncCompletion,
    ctx: *const ::core::ffi::c_void,
    synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn ConcurrentQueue_TryDequeueAsync(
            concurrentqueue: *const ::core::ffi::c_void,
            txn: *const ::core::ffi::c_void,
            timeout: i64,
            objecthandle: *mut usize,
            value: *mut Buffer,
            cts: *mut *mut ::core::ffi::c_void,
            succeeded: *mut ::windows::Win32::Foundation::BOOL,
            callback: fnNotifyTryDequeueAsyncCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    ConcurrentQueue_TryDequeueAsync(
        concurrentqueue,
        txn,
        timeout,
        objecthandle,
        value,
        cts,
        succeeded,
        callback,
        ctx,
        synchronouscomplete,
    )
    .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTxnReplicator<P0, P1, P2>(
    replicaid: i64,
    statefulservicepartition: *const ::core::ffi::c_void,
    datalosshandler: *const ::core::ffi::c_void,
    replicatorsettings: *const TxnReplicator_Settings,
    configpackagename: P0,
    replicatorsettingssectionname: P1,
    replicatorsecuritysectionname: P2,
    primaryreplicator: *mut *mut ::core::ffi::c_void,
    txnreplicatorhandle: *mut *mut ::core::ffi::c_void,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn GetTxnReplicator(
            replicaid: i64,
            statefulservicepartition: *const ::core::ffi::c_void,
            datalosshandler: *const ::core::ffi::c_void,
            replicatorsettings: *const TxnReplicator_Settings,
            configpackagename: ::windows_core::PCWSTR,
            replicatorsettingssectionname: ::windows_core::PCWSTR,
            replicatorsecuritysectionname: ::windows_core::PCWSTR,
            primaryreplicator: *mut *mut ::core::ffi::c_void,
            txnreplicatorhandle: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    GetTxnReplicator(
        replicaid,
        statefulservicepartition,
        datalosshandler,
        replicatorsettings,
        configpackagename.into_param().abi(),
        replicatorsettingssectionname.into_param().abi(),
        replicatorsecuritysectionname.into_param().abi(),
        primaryreplicator,
        txnreplicatorhandle,
    )
    .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrimaryReplicator_UpdateReplicatorSettings(
    primaryreplicator: *const ::core::ffi::c_void,
    replicatorsettings: *const TxnReplicator_Settings,
) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn PrimaryReplicator_UpdateReplicatorSettings(
            primaryreplicator: *const ::core::ffi::c_void,
            replicatorsettings: *const TxnReplicator_Settings,
        ) -> ::windows_core::HRESULT;
    }
    PrimaryReplicator_UpdateReplicatorSettings(primaryreplicator, replicatorsettings).ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn ReliableCollectionRuntime_Initialize(apiversion: u16) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn ReliableCollectionRuntime_Initialize(apiversion: u16) -> ::windows_core::HRESULT;
    }
    ReliableCollectionRuntime_Initialize(apiversion).ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReliableCollectionRuntime_Initialize2<P0>(
    apiversion: u16,
    standalonemode: P0,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows::Win32::Foundation::BOOL>,
{
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn ReliableCollectionRuntime_Initialize2(
            apiversion: u16,
            standalonemode: ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    ReliableCollectionRuntime_Initialize2(apiversion, standalonemode.into_param().abi()).ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn ReliableCollectionRuntime_StartTraceSessions() -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn ReliableCollectionRuntime_StartTraceSessions() -> ::windows_core::HRESULT;
    }
    ReliableCollectionRuntime_StartTraceSessions().ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn StateProviderEnumerator_AddRef(enumerator: *const ::core::ffi::c_void) {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn StateProviderEnumerator_AddRef(enumerator: *const ::core::ffi::c_void) -> ();
    }
    StateProviderEnumerator_AddRef(enumerator)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StateProviderEnumerator_MoveNext(
    enumerator: *const ::core::ffi::c_void,
    advanced: *mut ::windows::Win32::Foundation::BOOL,
    providername: *mut ::windows_core::PCWSTR,
    provider: *mut *mut ::core::ffi::c_void,
) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn StateProviderEnumerator_MoveNext(
            enumerator: *const ::core::ffi::c_void,
            advanced: *mut ::windows::Win32::Foundation::BOOL,
            providername: *mut ::windows_core::PCWSTR,
            provider: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    StateProviderEnumerator_MoveNext(enumerator, advanced, providername, provider).ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn StateProviderEnumerator_Release(enumerator: *const ::core::ffi::c_void) {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn StateProviderEnumerator_Release(enumerator: *const ::core::ffi::c_void) -> ();
    }
    StateProviderEnumerator_Release(enumerator)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn StateProvider_AddRef(stateproviderhandle: *const ::core::ffi::c_void) {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn StateProvider_AddRef(stateproviderhandle: *const ::core::ffi::c_void) -> ();
    }
    StateProvider_AddRef(stateproviderhandle)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn StateProvider_GetInfo<P0>(
    stateprovider: *const ::core::ffi::c_void,
    lang: P0,
) -> ::windows_core::Result<StateProvider_Info>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn StateProvider_GetInfo(
            stateprovider: *const ::core::ffi::c_void,
            lang: ::windows_core::PCWSTR,
            stateproviderinfo: *mut StateProvider_Info,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    StateProvider_GetInfo(stateprovider, lang.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn StateProvider_Release(stateproviderhandle: *const ::core::ffi::c_void) {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn StateProvider_Release(stateproviderhandle: *const ::core::ffi::c_void) -> ();
    }
    StateProvider_Release(stateproviderhandle)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn StoreKeyEnumerator_AddRef(enumerator: *mut ::core::ffi::c_void) {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn StoreKeyEnumerator_AddRef(enumerator: *mut ::core::ffi::c_void) -> ();
    }
    StoreKeyEnumerator_AddRef(enumerator)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StoreKeyEnumerator_MoveNext(
    enumerator: *const ::core::ffi::c_void,
    advanced: *mut ::windows::Win32::Foundation::BOOL,
    key: *mut ::windows_core::PCWSTR,
) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn StoreKeyEnumerator_MoveNext(
            enumerator: *const ::core::ffi::c_void,
            advanced: *mut ::windows::Win32::Foundation::BOOL,
            key: *mut ::windows_core::PCWSTR,
        ) -> ::windows_core::HRESULT;
    }
    StoreKeyEnumerator_MoveNext(enumerator, advanced, key).ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn StoreKeyEnumerator_Release(enumerator: *mut ::core::ffi::c_void) {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn StoreKeyEnumerator_Release(enumerator: *mut ::core::ffi::c_void) -> ();
    }
    StoreKeyEnumerator_Release(enumerator)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StoreKeyValueEnumerator_MoveNextAsync(
    enumerator: *const ::core::ffi::c_void,
    cts: *mut *mut ::core::ffi::c_void,
    advanced: *mut ::windows::Win32::Foundation::BOOL,
    key: *mut ::windows_core::PCWSTR,
    objecthandle: *mut usize,
    value: *mut Buffer,
    versionsequencenumber: *mut i64,
    callback: fnNotifyStoreKeyValueEnumeratorMoveNextAsyncCompletion,
    ctx: *const ::core::ffi::c_void,
    synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn StoreKeyValueEnumerator_MoveNextAsync(
            enumerator: *const ::core::ffi::c_void,
            cts: *mut *mut ::core::ffi::c_void,
            advanced: *mut ::windows::Win32::Foundation::BOOL,
            key: *mut ::windows_core::PCWSTR,
            objecthandle: *mut usize,
            value: *mut Buffer,
            versionsequencenumber: *mut i64,
            callback: fnNotifyStoreKeyValueEnumeratorMoveNextAsyncCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    StoreKeyValueEnumerator_MoveNextAsync(
        enumerator,
        cts,
        advanced,
        key,
        objecthandle,
        value,
        versionsequencenumber,
        callback,
        ctx,
        synchronouscomplete,
    )
    .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn StoreKeyValueEnumerator_Release(enumerator: *const ::core::ffi::c_void) {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn StoreKeyValueEnumerator_Release(enumerator: *const ::core::ffi::c_void) -> ();
    }
    StoreKeyValueEnumerator_Release(enumerator)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Store_AddAsync<P0>(
    store: *const ::core::ffi::c_void,
    txn: *const ::core::ffi::c_void,
    key: P0,
    objecthandle: usize,
    bytes: *const ::core::ffi::c_void,
    byteslength: u32,
    timeout: i64,
    cts: *mut *mut ::core::ffi::c_void,
    callback: fnNotifyAsyncCompletion,
    ctx: *const ::core::ffi::c_void,
    synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Store_AddAsync(
            store: *const ::core::ffi::c_void,
            txn: *const ::core::ffi::c_void,
            key: ::windows_core::PCWSTR,
            objecthandle: usize,
            bytes: *const ::core::ffi::c_void,
            byteslength: u32,
            timeout: i64,
            cts: *mut *mut ::core::ffi::c_void,
            callback: fnNotifyAsyncCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    Store_AddAsync(
        store,
        txn,
        key.into_param().abi(),
        objecthandle,
        bytes,
        byteslength,
        timeout,
        cts,
        callback,
        ctx,
        synchronouscomplete,
    )
    .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn Store_AddRef(storehandle: *const ::core::ffi::c_void) {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Store_AddRef(storehandle: *const ::core::ffi::c_void) -> ();
    }
    Store_AddRef(storehandle)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Store_ConditionalGetAsync<P0>(
    store: *const ::core::ffi::c_void,
    txn: *const ::core::ffi::c_void,
    key: P0,
    timeout: i64,
    lockmode: Store_LockMode,
    objecthandle: *mut usize,
    value: *mut Buffer,
    versionsequencenumber: *mut i64,
    cts: *mut *mut ::core::ffi::c_void,
    found: *mut ::windows::Win32::Foundation::BOOL,
    callback: fnNotifyGetAsyncCompletion,
    ctx: *const ::core::ffi::c_void,
    synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Store_ConditionalGetAsync(
            store: *const ::core::ffi::c_void,
            txn: *const ::core::ffi::c_void,
            key: ::windows_core::PCWSTR,
            timeout: i64,
            lockmode: Store_LockMode,
            objecthandle: *mut usize,
            value: *mut Buffer,
            versionsequencenumber: *mut i64,
            cts: *mut *mut ::core::ffi::c_void,
            found: *mut ::windows::Win32::Foundation::BOOL,
            callback: fnNotifyGetAsyncCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    Store_ConditionalGetAsync(
        store,
        txn,
        key.into_param().abi(),
        timeout,
        lockmode,
        objecthandle,
        value,
        versionsequencenumber,
        cts,
        found,
        callback,
        ctx,
        synchronouscomplete,
    )
    .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Store_ConditionalRemoveAsync<P0>(
    store: *const ::core::ffi::c_void,
    txn: *const ::core::ffi::c_void,
    key: P0,
    timeout: i64,
    cts: *mut *mut ::core::ffi::c_void,
    conditionalversion: i64,
    removed: *mut ::windows::Win32::Foundation::BOOL,
    callback: fnNotifyRemoveAsyncCompletion,
    ctx: *const ::core::ffi::c_void,
    synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Store_ConditionalRemoveAsync(
            store: *const ::core::ffi::c_void,
            txn: *const ::core::ffi::c_void,
            key: ::windows_core::PCWSTR,
            timeout: i64,
            cts: *mut *mut ::core::ffi::c_void,
            conditionalversion: i64,
            removed: *mut ::windows::Win32::Foundation::BOOL,
            callback: fnNotifyRemoveAsyncCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    Store_ConditionalRemoveAsync(
        store,
        txn,
        key.into_param().abi(),
        timeout,
        cts,
        conditionalversion,
        removed,
        callback,
        ctx,
        synchronouscomplete,
    )
    .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Store_ConditionalUpdateAsync<P0>(
    store: *const ::core::ffi::c_void,
    txn: *const ::core::ffi::c_void,
    key: P0,
    objecthandle: usize,
    bytes: *const ::core::ffi::c_void,
    byteslength: u32,
    timeout: i64,
    cts: *mut *mut ::core::ffi::c_void,
    conditionalversion: i64,
    updated: *mut ::windows::Win32::Foundation::BOOL,
    callback: fnNotifyUpdateAsyncCompletion,
    ctx: *const ::core::ffi::c_void,
    synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Store_ConditionalUpdateAsync(
            store: *const ::core::ffi::c_void,
            txn: *const ::core::ffi::c_void,
            key: ::windows_core::PCWSTR,
            objecthandle: usize,
            bytes: *const ::core::ffi::c_void,
            byteslength: u32,
            timeout: i64,
            cts: *mut *mut ::core::ffi::c_void,
            conditionalversion: i64,
            updated: *mut ::windows::Win32::Foundation::BOOL,
            callback: fnNotifyUpdateAsyncCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    Store_ConditionalUpdateAsync(
        store,
        txn,
        key.into_param().abi(),
        objecthandle,
        bytes,
        byteslength,
        timeout,
        cts,
        conditionalversion,
        updated,
        callback,
        ctx,
        synchronouscomplete,
    )
    .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Store_ContainsKeyAsync<P0>(
    store: *const ::core::ffi::c_void,
    txn: *const ::core::ffi::c_void,
    key: P0,
    timeout: i64,
    lockmode: Store_LockMode,
    cts: *mut *mut ::core::ffi::c_void,
    found: *mut ::windows::Win32::Foundation::BOOL,
    callback: fnNotifyContainsKeyAsyncCompletion,
    ctx: *const ::core::ffi::c_void,
    synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Store_ContainsKeyAsync(
            store: *const ::core::ffi::c_void,
            txn: *const ::core::ffi::c_void,
            key: ::windows_core::PCWSTR,
            timeout: i64,
            lockmode: Store_LockMode,
            cts: *mut *mut ::core::ffi::c_void,
            found: *mut ::windows::Win32::Foundation::BOOL,
            callback: fnNotifyContainsKeyAsyncCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    Store_ContainsKeyAsync(
        store,
        txn,
        key.into_param().abi(),
        timeout,
        lockmode,
        cts,
        found,
        callback,
        ctx,
        synchronouscomplete,
    )
    .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Store_CreateEnumeratorAsync(
    store: *const ::core::ffi::c_void,
    txn: *const ::core::ffi::c_void,
    enumerator: *mut *mut ::core::ffi::c_void,
    callback: fnNotifyCreateEnumeratorAsyncCompletion,
    ctx: *const ::core::ffi::c_void,
    synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Store_CreateEnumeratorAsync(
            store: *const ::core::ffi::c_void,
            txn: *const ::core::ffi::c_void,
            enumerator: *mut *mut ::core::ffi::c_void,
            callback: fnNotifyCreateEnumeratorAsyncCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    Store_CreateEnumeratorAsync(store, txn, enumerator, callback, ctx, synchronouscomplete).ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Store_CreateKeyEnumeratorAsync<P0, P1>(
    store: *const ::core::ffi::c_void,
    txn: *const ::core::ffi::c_void,
    firstkey: P0,
    lastkey: P1,
    enumerator: *mut *mut ::core::ffi::c_void,
    callback: fnNotifyCreateKeyEnumeratorAsyncCompletion,
    ctx: *const ::core::ffi::c_void,
    synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Store_CreateKeyEnumeratorAsync(
            store: *const ::core::ffi::c_void,
            txn: *const ::core::ffi::c_void,
            firstkey: ::windows_core::PCWSTR,
            lastkey: ::windows_core::PCWSTR,
            enumerator: *mut *mut ::core::ffi::c_void,
            callback: fnNotifyCreateKeyEnumeratorAsyncCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    Store_CreateKeyEnumeratorAsync(
        store,
        txn,
        firstkey.into_param().abi(),
        lastkey.into_param().abi(),
        enumerator,
        callback,
        ctx,
        synchronouscomplete,
    )
    .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Store_CreateRangedEnumeratorAsync<P0, P1>(
    store: *const ::core::ffi::c_void,
    txn: *const ::core::ffi::c_void,
    firstkey: P0,
    lastkey: P1,
    enumerator: *mut *mut ::core::ffi::c_void,
    callback: fnNotifyCreateEnumeratorAsyncCompletion,
    ctx: *const ::core::ffi::c_void,
    synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Store_CreateRangedEnumeratorAsync(
            store: *const ::core::ffi::c_void,
            txn: *const ::core::ffi::c_void,
            firstkey: ::windows_core::PCWSTR,
            lastkey: ::windows_core::PCWSTR,
            enumerator: *mut *mut ::core::ffi::c_void,
            callback: fnNotifyCreateEnumeratorAsyncCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    Store_CreateRangedEnumeratorAsync(
        store,
        txn,
        firstkey.into_param().abi(),
        lastkey.into_param().abi(),
        enumerator,
        callback,
        ctx,
        synchronouscomplete,
    )
    .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn Store_GetCount(store: *const ::core::ffi::c_void) -> ::windows_core::Result<i64> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Store_GetCount(
            store: *const ::core::ffi::c_void,
            count: *mut i64,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    Store_GetCount(store, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn Store_Release(storehandle: *const ::core::ffi::c_void) {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Store_Release(storehandle: *const ::core::ffi::c_void) -> ();
    }
    Store_Release(storehandle)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn Store_SetNotifyStoreChangeCallback(
    stateprovider: *const ::core::ffi::c_void,
    callback: fnNotifyStoreChangeCallback,
    cleanupcallback: fnCleanupContextCallback,
    ctx: *const ::core::ffi::c_void,
) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Store_SetNotifyStoreChangeCallback(
            stateprovider: *const ::core::ffi::c_void,
            callback: fnNotifyStoreChangeCallback,
            cleanupcallback: fnCleanupContextCallback,
            ctx: *const ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    Store_SetNotifyStoreChangeCallback(stateprovider, callback, cleanupcallback, ctx).ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn Store_SetNotifyStoreChangeCallbackMask(
    stateproviderhandle: *const ::core::ffi::c_void,
    mask: NotifyStoreChangeCallbackMask,
) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Store_SetNotifyStoreChangeCallbackMask(
            stateproviderhandle: *const ::core::ffi::c_void,
            mask: NotifyStoreChangeCallbackMask,
        ) -> ::windows_core::HRESULT;
    }
    Store_SetNotifyStoreChangeCallbackMask(stateproviderhandle, mask).ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Test_UseEnv<P0>(enable: P0)
where
    P0: ::windows_core::IntoParam<::windows::Win32::Foundation::BOOL>,
{
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Test_UseEnv(enable: ::windows::Win32::Foundation::BOOL) -> ();
    }
    Test_UseEnv(enable.into_param().abi())
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn Transaction_Abort(txn: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Transaction_Abort(txn: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
    }
    Transaction_Abort(txn).ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn Transaction_AddRef(txn: *const ::core::ffi::c_void) {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Transaction_AddRef(txn: *const ::core::ffi::c_void) -> ();
    }
    Transaction_AddRef(txn)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Transaction_CommitAsync(
    txn: *const ::core::ffi::c_void,
    callback: fnNotifyAsyncCompletion,
    ctx: *const ::core::ffi::c_void,
) -> ::windows_core::Result<::windows::Win32::Foundation::BOOL> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Transaction_CommitAsync(
            txn: *const ::core::ffi::c_void,
            callback: fnNotifyAsyncCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    Transaction_CommitAsync(txn, callback, ctx, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn Transaction_Dispose(txn: *const ::core::ffi::c_void) {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Transaction_Dispose(txn: *const ::core::ffi::c_void) -> ();
    }
    Transaction_Dispose(txn)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn Transaction_GetInfo(
    txnhandle: *const ::core::ffi::c_void,
    info: *mut Transaction_Info,
) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Transaction_GetInfo(
            txnhandle: *const ::core::ffi::c_void,
            info: *mut Transaction_Info,
        ) -> ::windows_core::HRESULT;
    }
    Transaction_GetInfo(txnhandle, info).ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Transaction_GetVisibilitySequenceNumberAsync(
    txnhandle: *const ::core::ffi::c_void,
    sequencenumber: *mut i64,
    callback: fnNotifyGetVisibilitySequenceNumberCompletion,
    ctx: *const ::core::ffi::c_void,
    synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Transaction_GetVisibilitySequenceNumberAsync(
            txnhandle: *const ::core::ffi::c_void,
            sequencenumber: *mut i64,
            callback: fnNotifyGetVisibilitySequenceNumberCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    Transaction_GetVisibilitySequenceNumberAsync(
        txnhandle,
        sequencenumber,
        callback,
        ctx,
        synchronouscomplete,
    )
    .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn Transaction_Release(txn: *const ::core::ffi::c_void) {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn Transaction_Release(txn: *const ::core::ffi::c_void) -> ();
    }
    Transaction_Release(txn)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxnReplicator_AddStateProviderAsync<P0, P1>(
    txnreplicator: *const ::core::ffi::c_void,
    txn: *const ::core::ffi::c_void,
    name: P0,
    lang: P1,
    stateproviderinfo: *const StateProvider_Info,
    timeout: i64,
    cts: *mut *mut ::core::ffi::c_void,
    callback: fnNotifyAsyncCompletion,
    ctx: *const ::core::ffi::c_void,
    synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn TxnReplicator_AddStateProviderAsync(
            txnreplicator: *const ::core::ffi::c_void,
            txn: *const ::core::ffi::c_void,
            name: ::windows_core::PCWSTR,
            lang: ::windows_core::PCWSTR,
            stateproviderinfo: *const StateProvider_Info,
            timeout: i64,
            cts: *mut *mut ::core::ffi::c_void,
            callback: fnNotifyAsyncCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    TxnReplicator_AddStateProviderAsync(
        txnreplicator,
        txn,
        name.into_param().abi(),
        lang.into_param().abi(),
        stateproviderinfo,
        timeout,
        cts,
        callback,
        ctx,
        synchronouscomplete,
    )
    .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxnReplicator_BackupAsync(
    txnreplicator: *const ::core::ffi::c_void,
    uploadasynccallback: fnUploadAsync,
    backupoption: Backup_Option,
    timeout: i64,
    cts: *mut *mut ::core::ffi::c_void,
    callback: fnNotifyAsyncCompletion,
    ctx: *const ::core::ffi::c_void,
    synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn TxnReplicator_BackupAsync(
            txnreplicator: *const ::core::ffi::c_void,
            uploadasynccallback: fnUploadAsync,
            backupoption: Backup_Option,
            timeout: i64,
            cts: *mut *mut ::core::ffi::c_void,
            callback: fnNotifyAsyncCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    TxnReplicator_BackupAsync(
        txnreplicator,
        uploadasynccallback,
        backupoption,
        timeout,
        cts,
        callback,
        ctx,
        synchronouscomplete,
    )
    .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxnReplicator_BackupAsync2(
    txnreplicator: *const ::core::ffi::c_void,
    uploadasynccallback: fnUploadAsync2,
    backupoption: Backup_Option,
    timeout: i64,
    cts: *mut *mut ::core::ffi::c_void,
    callback: fnNotifyAsyncCompletion,
    ctx: *const ::core::ffi::c_void,
    synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn TxnReplicator_BackupAsync2(
            txnreplicator: *const ::core::ffi::c_void,
            uploadasynccallback: fnUploadAsync2,
            backupoption: Backup_Option,
            timeout: i64,
            cts: *mut *mut ::core::ffi::c_void,
            callback: fnNotifyAsyncCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    TxnReplicator_BackupAsync2(
        txnreplicator,
        uploadasynccallback,
        backupoption,
        timeout,
        cts,
        callback,
        ctx,
        synchronouscomplete,
    )
    .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxnReplicator_CreateEnumerator<P0>(
    txnreplicator: *const ::core::ffi::c_void,
    parentsonly: P0,
    enumerator: *mut *mut ::core::ffi::c_void,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows::Win32::Foundation::BOOL>,
{
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn TxnReplicator_CreateEnumerator(
            txnreplicator: *const ::core::ffi::c_void,
            parentsonly: ::windows::Win32::Foundation::BOOL,
            enumerator: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    TxnReplicator_CreateEnumerator(txnreplicator, parentsonly.into_param().abi(), enumerator).ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn TxnReplicator_CreateTransaction(
    txnreplicator: *const ::core::ffi::c_void,
    txn: *mut *mut ::core::ffi::c_void,
) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn TxnReplicator_CreateTransaction(
            txnreplicator: *const ::core::ffi::c_void,
            txn: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    TxnReplicator_CreateTransaction(txnreplicator, txn).ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn TxnReplicator_GetInfo(
    txnreplicator: *const ::core::ffi::c_void,
    info: *mut TxnReplicator_Info,
) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn TxnReplicator_GetInfo(
            txnreplicator: *const ::core::ffi::c_void,
            info: *mut TxnReplicator_Info,
        ) -> ::windows_core::HRESULT;
    }
    TxnReplicator_GetInfo(txnreplicator, info).ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxnReplicator_GetOrAddStateProviderAsync<P0, P1>(
    txnreplicator: *const ::core::ffi::c_void,
    txn: *const ::core::ffi::c_void,
    name: P0,
    lang: P1,
    stateproviderinfo: *const StateProvider_Info,
    timeout: i64,
    cts: *mut *mut ::core::ffi::c_void,
    stateprovider: *mut *mut ::core::ffi::c_void,
    alreadyexist: *mut ::windows::Win32::Foundation::BOOL,
    callback: fnNotifyGetOrAddStateProviderAsyncCompletion,
    ctx: *const ::core::ffi::c_void,
    synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn TxnReplicator_GetOrAddStateProviderAsync(
            txnreplicator: *const ::core::ffi::c_void,
            txn: *const ::core::ffi::c_void,
            name: ::windows_core::PCWSTR,
            lang: ::windows_core::PCWSTR,
            stateproviderinfo: *const StateProvider_Info,
            timeout: i64,
            cts: *mut *mut ::core::ffi::c_void,
            stateprovider: *mut *mut ::core::ffi::c_void,
            alreadyexist: *mut ::windows::Win32::Foundation::BOOL,
            callback: fnNotifyGetOrAddStateProviderAsyncCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    TxnReplicator_GetOrAddStateProviderAsync(
        txnreplicator,
        txn,
        name.into_param().abi(),
        lang.into_param().abi(),
        stateproviderinfo,
        timeout,
        cts,
        stateprovider,
        alreadyexist,
        callback,
        ctx,
        synchronouscomplete,
    )
    .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn TxnReplicator_GetStateProvider<P0>(
    txnreplicator: *const ::core::ffi::c_void,
    name: P0,
    store: *mut *mut ::core::ffi::c_void,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn TxnReplicator_GetStateProvider(
            txnreplicator: *const ::core::ffi::c_void,
            name: ::windows_core::PCWSTR,
            store: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    TxnReplicator_GetStateProvider(txnreplicator, name.into_param().abi(), store).ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn TxnReplicator_Release(txnreplicator: *const ::core::ffi::c_void) {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn TxnReplicator_Release(txnreplicator: *const ::core::ffi::c_void) -> ();
    }
    TxnReplicator_Release(txnreplicator)
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxnReplicator_RemoveStateProviderAsync<P0>(
    txnreplicator: *const ::core::ffi::c_void,
    txn: *const ::core::ffi::c_void,
    name: P0,
    timeout: i64,
    cts: *mut *mut ::core::ffi::c_void,
    callback: fnNotifyAsyncCompletion,
    ctx: *const ::core::ffi::c_void,
    synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn TxnReplicator_RemoveStateProviderAsync(
            txnreplicator: *const ::core::ffi::c_void,
            txn: *const ::core::ffi::c_void,
            name: ::windows_core::PCWSTR,
            timeout: i64,
            cts: *mut *mut ::core::ffi::c_void,
            callback: fnNotifyAsyncCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    TxnReplicator_RemoveStateProviderAsync(
        txnreplicator,
        txn,
        name.into_param().abi(),
        timeout,
        cts,
        callback,
        ctx,
        synchronouscomplete,
    )
    .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxnReplicator_RestoreAsync<P0>(
    txnreplicator: *const ::core::ffi::c_void,
    backupfolder: P0,
    restorepolicy: Restore_Policy,
    timeout: i64,
    cts: *mut *mut ::core::ffi::c_void,
    callback: fnNotifyAsyncCompletion,
    ctx: *const ::core::ffi::c_void,
    synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn TxnReplicator_RestoreAsync(
            txnreplicator: *const ::core::ffi::c_void,
            backupfolder: ::windows_core::PCWSTR,
            restorepolicy: Restore_Policy,
            timeout: i64,
            cts: *mut *mut ::core::ffi::c_void,
            callback: fnNotifyAsyncCompletion,
            ctx: *const ::core::ffi::c_void,
            synchronouscomplete: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT;
    }
    TxnReplicator_RestoreAsync(
        txnreplicator,
        backupfolder.into_param().abi(),
        restorepolicy,
        timeout,
        cts,
        callback,
        ctx,
        synchronouscomplete,
    )
    .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn TxnReplicator_SetNotifyStateManagerChangeCallback(
    txnreplicator: *const ::core::ffi::c_void,
    callback: fnNotifyStateManagerChangeCallback,
    cleanupcallback: fnCleanupContextCallback,
    ctx: *const ::core::ffi::c_void,
) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn TxnReplicator_SetNotifyStateManagerChangeCallback(
            txnreplicator: *const ::core::ffi::c_void,
            callback: fnNotifyStateManagerChangeCallback,
            cleanupcallback: fnCleanupContextCallback,
            ctx: *const ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    TxnReplicator_SetNotifyStateManagerChangeCallback(txnreplicator, callback, cleanupcallback, ctx)
        .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[inline]
pub unsafe fn TxnReplicator_SetNotifyTransactionChangeCallback(
    txnreplicator: *const ::core::ffi::c_void,
    callback: fnNotifyTransactionChangeCallback,
    cleanupcallback: fnCleanupContextCallback,
    ctx: *const ::core::ffi::c_void,
) -> ::windows_core::Result<()> {
    #[link(name = "reliablecollectionruntime")]
    extern "system" {
        pub fn TxnReplicator_SetNotifyTransactionChangeCallback(
            txnreplicator: *const ::core::ffi::c_void,
            callback: fnNotifyTransactionChangeCallback,
            cleanupcallback: fnCleanupContextCallback,
            ctx: *const ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    TxnReplicator_SetNotifyTransactionChangeCallback(txnreplicator, callback, cleanupcallback, ctx)
        .ok()
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const Add: NotifyStoreChangeCallbackMask = NotifyStoreChangeCallbackMask(1u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const Backup_Option_Full: Backup_Option = Backup_Option(1u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const Backup_Option_Incremental: Backup_Option = Backup_Option(2u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const Backup_Option_Invalid: Backup_Option = Backup_Option(0u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const BatchAcknowledgementInterval: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(2u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const CheckpointThreshold: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(1048576u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const Clear: NotifyStoreChangeCallbackMask = NotifyStoreChangeCallbackMask(8u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const InitialCopyQueueSize: TxnReplicator_Settings_Flags = TxnReplicator_Settings_Flags(8u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const InitialPrimaryReplicationQueueSize: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(256u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const InitialSecondaryReplicationQueueSize: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(2048u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const MaxAccumulatedBackupLogSize: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(2097152u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const MaxCopyQueueSize: TxnReplicator_Settings_Flags = TxnReplicator_Settings_Flags(16u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const MaxMetadataSize: TxnReplicator_Settings_Flags = TxnReplicator_Settings_Flags(131072u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const MaxPrimaryReplicationQueueMemorySize: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(1024u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const MaxPrimaryReplicationQueueSize: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(512u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const MaxRecordSize: TxnReplicator_Settings_Flags = TxnReplicator_Settings_Flags(262144u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const MaxReplicationMessageSize: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(128u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const MaxSecondaryReplicationQueueMemorySize: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(8192u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const MaxSecondaryReplicationQueueSize: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(4096u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const MaxStreamSize: TxnReplicator_Settings_Flags = TxnReplicator_Settings_Flags(65536u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const MaxWriteQueueDepth: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(524288u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const MinLogSize: TxnReplicator_Settings_Flags = TxnReplicator_Settings_Flags(33554432u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const None: TxnReplicator_Settings_Flags = TxnReplicator_Settings_Flags(0u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const OptimizeForLocalSSD: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(4194304u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const OptimizeLogForLowerDiskUsage: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(8388608u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const RELIABLECOLLECTION_API_VERSION: u32 = 256u32;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const Rebuild: NotifyStoreChangeCallbackMask = NotifyStoreChangeCallbackMask(16u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const Remove: NotifyStoreChangeCallbackMask = NotifyStoreChangeCallbackMask(4u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const ReplicatorAddress: TxnReplicator_Settings_Flags = TxnReplicator_Settings_Flags(4u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const ReplicatorListenAddress: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(16384u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const ReplicatorPublishAddress: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(32768u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const Restore_Policy_Invalid: Restore_Policy = Restore_Policy(0u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const Restore_Policy_Safe: Restore_Policy = Restore_Policy(1u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const Restore_policy_Force: Restore_Policy = Restore_Policy(2u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const RetryInterval: TxnReplicator_Settings_Flags = TxnReplicator_Settings_Flags(1u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const SecondaryClearAcknowledgedOperations: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(64u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const SecurityCredentials: TxnReplicator_Settings_Flags = TxnReplicator_Settings_Flags(32u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const SharedLogId: TxnReplicator_Settings_Flags = TxnReplicator_Settings_Flags(268435456u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const SharedLogPath: TxnReplicator_Settings_Flags = TxnReplicator_Settings_Flags(536870912u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const SlowApiMonitoringDuration: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(16777216u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const StateManagerChangeKind_Add: StateManagerChangeKind = StateManagerChangeKind(0u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const StateManagerChangeKind_Rebuild: StateManagerChangeKind = StateManagerChangeKind(2u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const StateManagerChangeKind_Remove: StateManagerChangeKind = StateManagerChangeKind(1u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const StateProvider_Info_V1_Size: u32 = 16u32;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const StateProvider_Kind_ConcurrentQueue: StateProvider_Kind = StateProvider_Kind(2u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const StateProvider_Kind_Invalid: StateProvider_Kind = StateProvider_Kind(0u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const StateProvider_Kind_ReliableDictionary_Compat: StateProvider_Kind =
    StateProvider_Kind(3u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const StateProvider_Kind_Store: StateProvider_Kind = StateProvider_Kind(1u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const StoreChangeKind_Add: StoreChangeKind = StoreChangeKind(0u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const StoreChangeKind_Clear: StoreChangeKind = StoreChangeKind(3u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const StoreChangeKind_Rebuild: StoreChangeKind = StoreChangeKind(4u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const StoreChangeKind_Remove: StoreChangeKind = StoreChangeKind(2u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const StoreChangeKind_Update: StoreChangeKind = StoreChangeKind(1u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const Store_LockMode_Exclusive: Store_LockMode = Store_LockMode(2u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const Store_LockMode_Free: Store_LockMode = Store_LockMode(0u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const Store_LockMode_Shared: Store_LockMode = Store_LockMode(1u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const Store_LockMode_Update: Store_LockMode = Store_LockMode(3u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const ThrottlingThresholdFactor: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(134217728u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const TransactionChangeKind_Commit: TransactionChangeKind = TransactionChangeKind(0u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const TruncationThresholdFactor: TxnReplicator_Settings_Flags =
    TxnReplicator_Settings_Flags(67108864u64);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub const Update: NotifyStoreChangeCallbackMask = NotifyStoreChangeCallbackMask(2u32);
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Backup_Option(pub u32);
impl ::core::marker::Copy for Backup_Option {}
impl ::core::clone::Clone for Backup_Option {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Backup_Option {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for Backup_Option {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for Backup_Option {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Backup_Option").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NotifyStoreChangeCallbackMask(pub u32);
impl ::core::marker::Copy for NotifyStoreChangeCallbackMask {}
impl ::core::clone::Clone for NotifyStoreChangeCallbackMask {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NotifyStoreChangeCallbackMask {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NotifyStoreChangeCallbackMask {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NotifyStoreChangeCallbackMask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotifyStoreChangeCallbackMask")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Restore_Policy(pub u32);
impl ::core::marker::Copy for Restore_Policy {}
impl ::core::clone::Clone for Restore_Policy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Restore_Policy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for Restore_Policy {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for Restore_Policy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Restore_Policy").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StateManagerChangeKind(pub u32);
impl ::core::marker::Copy for StateManagerChangeKind {}
impl ::core::clone::Clone for StateManagerChangeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StateManagerChangeKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for StateManagerChangeKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for StateManagerChangeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StateManagerChangeKind")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StateProvider_Kind(pub u32);
impl ::core::marker::Copy for StateProvider_Kind {}
impl ::core::clone::Clone for StateProvider_Kind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StateProvider_Kind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for StateProvider_Kind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for StateProvider_Kind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StateProvider_Kind").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StoreChangeKind(pub u32);
impl ::core::marker::Copy for StoreChangeKind {}
impl ::core::clone::Clone for StoreChangeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StoreChangeKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for StoreChangeKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for StoreChangeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreChangeKind").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Store_LockMode(pub u32);
impl ::core::marker::Copy for Store_LockMode {}
impl ::core::clone::Clone for Store_LockMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Store_LockMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for Store_LockMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for Store_LockMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Store_LockMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TransactionChangeKind(pub u32);
impl ::core::marker::Copy for TransactionChangeKind {}
impl ::core::clone::Clone for TransactionChangeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TransactionChangeKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TransactionChangeKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TransactionChangeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TransactionChangeKind")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TxnReplicator_Settings_Flags(pub u64);
impl ::core::marker::Copy for TxnReplicator_Settings_Flags {}
impl ::core::clone::Clone for TxnReplicator_Settings_Flags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TxnReplicator_Settings_Flags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TxnReplicator_Settings_Flags {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TxnReplicator_Settings_Flags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TxnReplicator_Settings_Flags")
            .field(&self.0)
            .finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub struct Backup_Info {
    pub backupId: ::windows_core::GUID,
    pub directoryPath: ::windows_core::PCWSTR,
    pub option: Backup_Option,
    pub version: Backup_Version,
}
impl ::core::marker::Copy for Backup_Info {}
impl ::core::clone::Clone for Backup_Info {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Backup_Info {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Backup_Info")
            .field("backupId", &self.backupId)
            .field("directoryPath", &self.directoryPath)
            .field("option", &self.option)
            .field("version", &self.version)
            .finish()
    }
}
impl ::windows_core::TypeKind for Backup_Info {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for Backup_Info {
    fn eq(&self, other: &Self) -> bool {
        self.backupId == other.backupId
            && self.directoryPath == other.directoryPath
            && self.option == other.option
            && self.version == other.version
    }
}
impl ::core::cmp::Eq for Backup_Info {}
impl ::core::default::Default for Backup_Info {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub struct Backup_Info2 {
    pub Size: u32,
    pub backupId: ::windows_core::GUID,
    pub directoryPath: ::windows_core::PCWSTR,
    pub option: Backup_Option,
    pub version: Backup_Version,
    pub startVersion: Backup_Version,
    pub parentbackupId: ::windows_core::GUID,
}
impl ::core::marker::Copy for Backup_Info2 {}
impl ::core::clone::Clone for Backup_Info2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Backup_Info2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Backup_Info2")
            .field("Size", &self.Size)
            .field("backupId", &self.backupId)
            .field("directoryPath", &self.directoryPath)
            .field("option", &self.option)
            .field("version", &self.version)
            .field("startVersion", &self.startVersion)
            .field("parentbackupId", &self.parentbackupId)
            .finish()
    }
}
impl ::windows_core::TypeKind for Backup_Info2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for Backup_Info2 {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.backupId == other.backupId
            && self.directoryPath == other.directoryPath
            && self.option == other.option
            && self.version == other.version
            && self.startVersion == other.startVersion
            && self.parentbackupId == other.parentbackupId
    }
}
impl ::core::cmp::Eq for Backup_Info2 {}
impl ::core::default::Default for Backup_Info2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub struct Backup_Version {
    pub epoch: Epoch,
    pub lsn: i64,
}
impl ::core::marker::Copy for Backup_Version {}
impl ::core::clone::Clone for Backup_Version {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Backup_Version {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Backup_Version")
            .field("epoch", &self.epoch)
            .field("lsn", &self.lsn)
            .finish()
    }
}
impl ::windows_core::TypeKind for Backup_Version {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for Backup_Version {
    fn eq(&self, other: &Self) -> bool {
        self.epoch == other.epoch && self.lsn == other.lsn
    }
}
impl ::core::cmp::Eq for Backup_Version {}
impl ::core::default::Default for Backup_Version {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub struct Buffer {
    pub Bytes: ::windows_core::PSTR,
    pub Length: u32,
    pub Handle: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for Buffer {}
impl ::core::clone::Clone for Buffer {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Buffer")
            .field("Bytes", &self.Bytes)
            .field("Length", &self.Length)
            .field("Handle", &self.Handle)
            .finish()
    }
}
impl ::windows_core::TypeKind for Buffer {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for Buffer {
    fn eq(&self, other: &Self) -> bool {
        self.Bytes == other.Bytes && self.Length == other.Length && self.Handle == other.Handle
    }
}
impl ::core::cmp::Eq for Buffer {}
impl ::core::default::Default for Buffer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub struct Epoch {
    pub DataLossNumber: i64,
    pub ConfigurationNumber: i64,
    pub Reserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for Epoch {}
impl ::core::clone::Clone for Epoch {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Epoch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Epoch")
            .field("DataLossNumber", &self.DataLossNumber)
            .field("ConfigurationNumber", &self.ConfigurationNumber)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::windows_core::TypeKind for Epoch {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for Epoch {
    fn eq(&self, other: &Self) -> bool {
        self.DataLossNumber == other.DataLossNumber
            && self.ConfigurationNumber == other.ConfigurationNumber
            && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for Epoch {}
impl ::core::default::Default for Epoch {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub struct StateManagerChangeData_Rebuild {
    pub StateProviders: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for StateManagerChangeData_Rebuild {}
impl ::core::clone::Clone for StateManagerChangeData_Rebuild {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for StateManagerChangeData_Rebuild {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StateManagerChangeData_Rebuild")
            .field("StateProviders", &self.StateProviders)
            .finish()
    }
}
impl ::windows_core::TypeKind for StateManagerChangeData_Rebuild {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for StateManagerChangeData_Rebuild {
    fn eq(&self, other: &Self) -> bool {
        self.StateProviders == other.StateProviders
    }
}
impl ::core::cmp::Eq for StateManagerChangeData_Rebuild {}
impl ::core::default::Default for StateManagerChangeData_Rebuild {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub struct StateManagerChangeData_SingleEntityChanged {
    pub Transaction: *mut ::core::ffi::c_void,
    pub StateProvider: *mut ::core::ffi::c_void,
    pub StateProviderName: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for StateManagerChangeData_SingleEntityChanged {}
impl ::core::clone::Clone for StateManagerChangeData_SingleEntityChanged {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for StateManagerChangeData_SingleEntityChanged {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StateManagerChangeData_SingleEntityChanged")
            .field("Transaction", &self.Transaction)
            .field("StateProvider", &self.StateProvider)
            .field("StateProviderName", &self.StateProviderName)
            .finish()
    }
}
impl ::windows_core::TypeKind for StateManagerChangeData_SingleEntityChanged {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for StateManagerChangeData_SingleEntityChanged {
    fn eq(&self, other: &Self) -> bool {
        self.Transaction == other.Transaction
            && self.StateProvider == other.StateProvider
            && self.StateProviderName == other.StateProviderName
    }
}
impl ::core::cmp::Eq for StateManagerChangeData_SingleEntityChanged {}
impl ::core::default::Default for StateManagerChangeData_SingleEntityChanged {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub struct StateProvider_Info {
    pub Size: u32,
    pub Kind: StateProvider_Kind,
    pub LangMetadata: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for StateProvider_Info {}
impl ::core::clone::Clone for StateProvider_Info {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for StateProvider_Info {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StateProvider_Info")
            .field("Size", &self.Size)
            .field("Kind", &self.Kind)
            .field("LangMetadata", &self.LangMetadata)
            .finish()
    }
}
impl ::windows_core::TypeKind for StateProvider_Info {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for StateProvider_Info {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Kind == other.Kind
            && self.LangMetadata == other.LangMetadata
    }
}
impl ::core::cmp::Eq for StateProvider_Info {}
impl ::core::default::Default for StateProvider_Info {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub struct StoreChangeData_Add {
    pub CommitSequnceNumber: i64,
    pub Transaction: *mut ::core::ffi::c_void,
    pub Key: ::windows_core::PCWSTR,
    pub Value: ::windows_core::PSTR,
    pub Length: u32,
}
impl ::core::marker::Copy for StoreChangeData_Add {}
impl ::core::clone::Clone for StoreChangeData_Add {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for StoreChangeData_Add {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StoreChangeData_Add")
            .field("CommitSequnceNumber", &self.CommitSequnceNumber)
            .field("Transaction", &self.Transaction)
            .field("Key", &self.Key)
            .field("Value", &self.Value)
            .field("Length", &self.Length)
            .finish()
    }
}
impl ::windows_core::TypeKind for StoreChangeData_Add {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for StoreChangeData_Add {
    fn eq(&self, other: &Self) -> bool {
        self.CommitSequnceNumber == other.CommitSequnceNumber
            && self.Transaction == other.Transaction
            && self.Key == other.Key
            && self.Value == other.Value
            && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for StoreChangeData_Add {}
impl ::core::default::Default for StoreChangeData_Add {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub struct StoreChangeData_Rebuild {
    pub Enumerator: *mut ::core::ffi::c_void,
    pub Callback: fnAsyncCompletionCallback,
    pub Context: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for StoreChangeData_Rebuild {}
impl ::core::clone::Clone for StoreChangeData_Rebuild {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for StoreChangeData_Rebuild {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StoreChangeData_Rebuild")
            .field("Enumerator", &self.Enumerator)
            .field("Context", &self.Context)
            .finish()
    }
}
impl ::windows_core::TypeKind for StoreChangeData_Rebuild {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for StoreChangeData_Rebuild {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub struct StoreChangeData_Remove {
    pub CommitSequnceNumber: i64,
    pub Transaction: *mut ::core::ffi::c_void,
    pub Key: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for StoreChangeData_Remove {}
impl ::core::clone::Clone for StoreChangeData_Remove {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for StoreChangeData_Remove {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StoreChangeData_Remove")
            .field("CommitSequnceNumber", &self.CommitSequnceNumber)
            .field("Transaction", &self.Transaction)
            .field("Key", &self.Key)
            .finish()
    }
}
impl ::windows_core::TypeKind for StoreChangeData_Remove {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for StoreChangeData_Remove {
    fn eq(&self, other: &Self) -> bool {
        self.CommitSequnceNumber == other.CommitSequnceNumber
            && self.Transaction == other.Transaction
            && self.Key == other.Key
    }
}
impl ::core::cmp::Eq for StoreChangeData_Remove {}
impl ::core::default::Default for StoreChangeData_Remove {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub struct StoreChangeData_Update {
    pub CommitSequnceNumber: i64,
    pub Transaction: *mut ::core::ffi::c_void,
    pub Key: ::windows_core::PCWSTR,
    pub Value: ::windows_core::PSTR,
    pub Length: u32,
}
impl ::core::marker::Copy for StoreChangeData_Update {}
impl ::core::clone::Clone for StoreChangeData_Update {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for StoreChangeData_Update {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StoreChangeData_Update")
            .field("CommitSequnceNumber", &self.CommitSequnceNumber)
            .field("Transaction", &self.Transaction)
            .field("Key", &self.Key)
            .field("Value", &self.Value)
            .field("Length", &self.Length)
            .finish()
    }
}
impl ::windows_core::TypeKind for StoreChangeData_Update {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for StoreChangeData_Update {
    fn eq(&self, other: &Self) -> bool {
        self.CommitSequnceNumber == other.CommitSequnceNumber
            && self.Transaction == other.Transaction
            && self.Key == other.Key
            && self.Value == other.Value
            && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for StoreChangeData_Update {}
impl ::core::default::Default for StoreChangeData_Update {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub struct TransactionChangeData_Commit {
    pub Transaction: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for TransactionChangeData_Commit {}
impl ::core::clone::Clone for TransactionChangeData_Commit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TransactionChangeData_Commit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TransactionChangeData_Commit")
            .field("Transaction", &self.Transaction)
            .finish()
    }
}
impl ::windows_core::TypeKind for TransactionChangeData_Commit {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for TransactionChangeData_Commit {
    fn eq(&self, other: &Self) -> bool {
        self.Transaction == other.Transaction
    }
}
impl ::core::cmp::Eq for TransactionChangeData_Commit {}
impl ::core::default::Default for TransactionChangeData_Commit {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub struct Transaction_Info {
    pub Size: u32,
    pub CommitSequenceNumber: i64,
    pub Id: i64,
}
impl ::core::marker::Copy for Transaction_Info {}
impl ::core::clone::Clone for Transaction_Info {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Transaction_Info {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Transaction_Info")
            .field("Size", &self.Size)
            .field("CommitSequenceNumber", &self.CommitSequenceNumber)
            .field("Id", &self.Id)
            .finish()
    }
}
impl ::windows_core::TypeKind for Transaction_Info {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for Transaction_Info {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.CommitSequenceNumber == other.CommitSequenceNumber
            && self.Id == other.Id
    }
}
impl ::core::cmp::Eq for Transaction_Info {}
impl ::core::default::Default for Transaction_Info {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub struct TxnReplicator_Info {
    pub Size: u32,
    pub LastStableSequenceNumber: i64,
    pub LastCommittedSequenceNumber: i64,
    pub CurrentEpoch: Epoch,
}
impl ::core::marker::Copy for TxnReplicator_Info {}
impl ::core::clone::Clone for TxnReplicator_Info {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TxnReplicator_Info {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TxnReplicator_Info")
            .field("Size", &self.Size)
            .field("LastStableSequenceNumber", &self.LastStableSequenceNumber)
            .field(
                "LastCommittedSequenceNumber",
                &self.LastCommittedSequenceNumber,
            )
            .field("CurrentEpoch", &self.CurrentEpoch)
            .finish()
    }
}
impl ::windows_core::TypeKind for TxnReplicator_Info {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for TxnReplicator_Info {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.LastStableSequenceNumber == other.LastStableSequenceNumber
            && self.LastCommittedSequenceNumber == other.LastCommittedSequenceNumber
            && self.CurrentEpoch == other.CurrentEpoch
    }
}
impl ::core::cmp::Eq for TxnReplicator_Info {}
impl ::core::default::Default for TxnReplicator_Info {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TxnReplicator_Settings {
    pub Flags: u64,
    pub RetryIntervalMilliseconds: u32,
    pub BatchAcknowledgementIntervalMilliseconds: u32,
    pub ReplicatorAddress: *mut ::core::ffi::c_void,
    pub InitialCopyQueueSize: u32,
    pub MaxCopyQueueSize: u32,
    pub SecurityCredentials: *mut ::core::ffi::c_void,
    pub SecondaryClearAcknowledgedOperations: ::windows::Win32::Foundation::BOOL,
    pub MaxReplicationMessageSize: u32,
    pub InitialPrimaryReplicationQueueSize: u32,
    pub MaxPrimaryReplicationQueueSize: u32,
    pub MaxPrimaryReplicationQueueMemorySize: u32,
    pub InitialSecondaryReplicationQueueSize: u32,
    pub MaxSecondaryReplicationQueueSize: u32,
    pub MaxSecondaryReplicationQueueMemorySize: u32,
    pub ReplicatorListenAddress: *mut ::core::ffi::c_void,
    pub ReplicatorPublishAddress: *mut ::core::ffi::c_void,
    pub MaxStreamSizeInMB: u32,
    pub MaxMetadataSizeInKB: u32,
    pub MaxRecordSizeInKB: u32,
    pub MaxWriteQueueDepthInKB: u32,
    pub CheckpointThresholdInMB: u32,
    pub MaxAccumulatedBackupLogSizeInMB: u32,
    pub OptimizeForLocalSSD: ::windows::Win32::Foundation::BOOL,
    pub OptimizeLogForLowerDiskUsage: ::windows::Win32::Foundation::BOOL,
    pub SlowApiMonitoringDurationSeconds: u32,
    pub MinLogSizeInMB: u32,
    pub TruncationThresholdFactor: u32,
    pub ThrottlingThresholdFactor: u32,
    pub SharedLogId: *mut ::core::ffi::c_void,
    pub SharedLogPath: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TxnReplicator_Settings {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TxnReplicator_Settings {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TxnReplicator_Settings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TxnReplicator_Settings")
            .field("Flags", &self.Flags)
            .field("RetryIntervalMilliseconds", &self.RetryIntervalMilliseconds)
            .field(
                "BatchAcknowledgementIntervalMilliseconds",
                &self.BatchAcknowledgementIntervalMilliseconds,
            )
            .field("ReplicatorAddress", &self.ReplicatorAddress)
            .field("InitialCopyQueueSize", &self.InitialCopyQueueSize)
            .field("MaxCopyQueueSize", &self.MaxCopyQueueSize)
            .field("SecurityCredentials", &self.SecurityCredentials)
            .field(
                "SecondaryClearAcknowledgedOperations",
                &self.SecondaryClearAcknowledgedOperations,
            )
            .field("MaxReplicationMessageSize", &self.MaxReplicationMessageSize)
            .field(
                "InitialPrimaryReplicationQueueSize",
                &self.InitialPrimaryReplicationQueueSize,
            )
            .field(
                "MaxPrimaryReplicationQueueSize",
                &self.MaxPrimaryReplicationQueueSize,
            )
            .field(
                "MaxPrimaryReplicationQueueMemorySize",
                &self.MaxPrimaryReplicationQueueMemorySize,
            )
            .field(
                "InitialSecondaryReplicationQueueSize",
                &self.InitialSecondaryReplicationQueueSize,
            )
            .field(
                "MaxSecondaryReplicationQueueSize",
                &self.MaxSecondaryReplicationQueueSize,
            )
            .field(
                "MaxSecondaryReplicationQueueMemorySize",
                &self.MaxSecondaryReplicationQueueMemorySize,
            )
            .field("ReplicatorListenAddress", &self.ReplicatorListenAddress)
            .field("ReplicatorPublishAddress", &self.ReplicatorPublishAddress)
            .field("MaxStreamSizeInMB", &self.MaxStreamSizeInMB)
            .field("MaxMetadataSizeInKB", &self.MaxMetadataSizeInKB)
            .field("MaxRecordSizeInKB", &self.MaxRecordSizeInKB)
            .field("MaxWriteQueueDepthInKB", &self.MaxWriteQueueDepthInKB)
            .field("CheckpointThresholdInMB", &self.CheckpointThresholdInMB)
            .field(
                "MaxAccumulatedBackupLogSizeInMB",
                &self.MaxAccumulatedBackupLogSizeInMB,
            )
            .field("OptimizeForLocalSSD", &self.OptimizeForLocalSSD)
            .field(
                "OptimizeLogForLowerDiskUsage",
                &self.OptimizeLogForLowerDiskUsage,
            )
            .field(
                "SlowApiMonitoringDurationSeconds",
                &self.SlowApiMonitoringDurationSeconds,
            )
            .field("MinLogSizeInMB", &self.MinLogSizeInMB)
            .field("TruncationThresholdFactor", &self.TruncationThresholdFactor)
            .field("ThrottlingThresholdFactor", &self.ThrottlingThresholdFactor)
            .field("SharedLogId", &self.SharedLogId)
            .field("SharedLogPath", &self.SharedLogPath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for TxnReplicator_Settings {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TxnReplicator_Settings {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
            && self.RetryIntervalMilliseconds == other.RetryIntervalMilliseconds
            && self.BatchAcknowledgementIntervalMilliseconds
                == other.BatchAcknowledgementIntervalMilliseconds
            && self.ReplicatorAddress == other.ReplicatorAddress
            && self.InitialCopyQueueSize == other.InitialCopyQueueSize
            && self.MaxCopyQueueSize == other.MaxCopyQueueSize
            && self.SecurityCredentials == other.SecurityCredentials
            && self.SecondaryClearAcknowledgedOperations
                == other.SecondaryClearAcknowledgedOperations
            && self.MaxReplicationMessageSize == other.MaxReplicationMessageSize
            && self.InitialPrimaryReplicationQueueSize == other.InitialPrimaryReplicationQueueSize
            && self.MaxPrimaryReplicationQueueSize == other.MaxPrimaryReplicationQueueSize
            && self.MaxPrimaryReplicationQueueMemorySize
                == other.MaxPrimaryReplicationQueueMemorySize
            && self.InitialSecondaryReplicationQueueSize
                == other.InitialSecondaryReplicationQueueSize
            && self.MaxSecondaryReplicationQueueSize == other.MaxSecondaryReplicationQueueSize
            && self.MaxSecondaryReplicationQueueMemorySize
                == other.MaxSecondaryReplicationQueueMemorySize
            && self.ReplicatorListenAddress == other.ReplicatorListenAddress
            && self.ReplicatorPublishAddress == other.ReplicatorPublishAddress
            && self.MaxStreamSizeInMB == other.MaxStreamSizeInMB
            && self.MaxMetadataSizeInKB == other.MaxMetadataSizeInKB
            && self.MaxRecordSizeInKB == other.MaxRecordSizeInKB
            && self.MaxWriteQueueDepthInKB == other.MaxWriteQueueDepthInKB
            && self.CheckpointThresholdInMB == other.CheckpointThresholdInMB
            && self.MaxAccumulatedBackupLogSizeInMB == other.MaxAccumulatedBackupLogSizeInMB
            && self.OptimizeForLocalSSD == other.OptimizeForLocalSSD
            && self.OptimizeLogForLowerDiskUsage == other.OptimizeLogForLowerDiskUsage
            && self.SlowApiMonitoringDurationSeconds == other.SlowApiMonitoringDurationSeconds
            && self.MinLogSizeInMB == other.MinLogSizeInMB
            && self.TruncationThresholdFactor == other.TruncationThresholdFactor
            && self.ThrottlingThresholdFactor == other.ThrottlingThresholdFactor
            && self.SharedLogId == other.SharedLogId
            && self.SharedLogPath == other.SharedLogPath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TxnReplicator_Settings {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TxnReplicator_Settings {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub type fnAsyncCompletionCallback =
    ::core::option::Option<unsafe extern "system" fn(ctx: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub type fnCleanupContextCallback =
    ::core::option::Option<unsafe extern "system" fn(ctx: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub type fnNotifyAsyncCompletion = ::core::option::Option<
    unsafe extern "system" fn(ctx: *mut ::core::ffi::c_void, status: ::windows_core::HRESULT) -> (),
>;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type fnNotifyContainsKeyAsyncCompletion = ::core::option::Option<
    unsafe extern "system" fn(
        ctx: *mut ::core::ffi::c_void,
        status: ::windows_core::HRESULT,
        found: ::windows::Win32::Foundation::BOOL,
    ) -> (),
>;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub type fnNotifyCreateEnumeratorAsyncCompletion = ::core::option::Option<
    unsafe extern "system" fn(
        ctx: *mut ::core::ffi::c_void,
        status: ::windows_core::HRESULT,
        enumerator: *mut ::core::ffi::c_void,
    ) -> (),
>;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub type fnNotifyCreateKeyEnumeratorAsyncCompletion = ::core::option::Option<
    unsafe extern "system" fn(
        ctx: *mut ::core::ffi::c_void,
        status: ::windows_core::HRESULT,
        enumerator: *mut ::core::ffi::c_void,
    ) -> (),
>;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type fnNotifyGetAsyncCompletion = ::core::option::Option<
    unsafe extern "system" fn(
        ctx: *mut ::core::ffi::c_void,
        status: ::windows_core::HRESULT,
        r: ::windows::Win32::Foundation::BOOL,
        objecthandle: usize,
        bytes: *mut ::core::ffi::c_void,
        byteslength: u32,
        versionsequencenumber: i64,
    ) -> (),
>;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type fnNotifyGetOrAddStateProviderAsyncCompletion = ::core::option::Option<
    unsafe extern "system" fn(
        ctx: *mut ::core::ffi::c_void,
        status: ::windows_core::HRESULT,
        store: *mut ::core::ffi::c_void,
        exist: ::windows::Win32::Foundation::BOOL,
    ) -> (),
>;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub type fnNotifyGetVisibilitySequenceNumberCompletion = ::core::option::Option<
    unsafe extern "system" fn(
        ctx: *const ::core::ffi::c_void,
        status: ::windows_core::HRESULT,
        vsn: i64,
    ) -> (),
>;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type fnNotifyRemoveAsyncCompletion = ::core::option::Option<
    unsafe extern "system" fn(
        ctx: *mut ::core::ffi::c_void,
        status: ::windows_core::HRESULT,
        removed: ::windows::Win32::Foundation::BOOL,
    ) -> (),
>;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub type fnNotifyStateManagerChangeCallback = ::core::option::Option<
    unsafe extern "system" fn(
        ctx: *mut ::core::ffi::c_void,
        txnreplicator: *mut ::core::ffi::c_void,
        changekind: StateManagerChangeKind,
        pdata: *mut ::core::ffi::c_void,
    ) -> (),
>;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub type fnNotifyStoreChangeCallback = ::core::option::Option<
    unsafe extern "system" fn(
        ctx: *mut ::core::ffi::c_void,
        stateprovider: *mut ::core::ffi::c_void,
        storechangekind: StoreChangeKind,
        pdata: *mut ::core::ffi::c_void,
    ) -> (),
>;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type fnNotifyStoreKeyValueEnumeratorMoveNextAsyncCompletion = ::core::option::Option<
    unsafe extern "system" fn(
        ctx: *mut ::core::ffi::c_void,
        status: ::windows_core::HRESULT,
        advanced: ::windows::Win32::Foundation::BOOL,
        key: ::windows_core::PCWSTR,
        objecthandle: usize,
        bytebuffer: *mut ::core::ffi::c_void,
        bufferlength: u32,
        versionsequencenumber: i64,
    ) -> (),
>;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`*"]
pub type fnNotifyTransactionChangeCallback = ::core::option::Option<
    unsafe extern "system" fn(
        ctx: *mut ::core::ffi::c_void,
        txnreplicator: *mut ::core::ffi::c_void,
        changekind: TransactionChangeKind,
        pdata: *mut ::core::ffi::c_void,
    ) -> (),
>;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type fnNotifyTryDequeueAsyncCompletion = ::core::option::Option<
    unsafe extern "system" fn(
        ctx: *mut ::core::ffi::c_void,
        status: ::windows_core::HRESULT,
        succeeded: ::windows::Win32::Foundation::BOOL,
        objecthandle: usize,
        bytes: *mut ::core::ffi::c_void,
        byteslength: u32,
    ) -> (),
>;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type fnNotifyUpdateAsyncCompletion = ::core::option::Option<
    unsafe extern "system" fn(
        ctx: *mut ::core::ffi::c_void,
        status: ::windows_core::HRESULT,
        updated: ::windows::Win32::Foundation::BOOL,
    ) -> (),
>;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type fnNotifyUploadAsyncCompletion = ::core::option::Option<
    unsafe extern "system" fn(
        ctx: *mut ::core::ffi::c_void,
        uploaded: ::windows::Win32::Foundation::BOOL,
    ) -> (),
>;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type fnUploadAsync = ::core::option::Option<
    unsafe extern "system" fn(
        ctx: *mut ::core::ffi::c_void,
        backup_info: Backup_Info,
        uploadcallbackend: fnNotifyUploadAsyncCompletion,
        uploadasynccompletionctx: *mut ::core::ffi::c_void,
    ) -> (),
>;
#[doc = "*Required features: `\"ServiceFabric_ReliableCollectionRuntime\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type fnUploadAsync2 = ::core::option::Option<
    unsafe extern "system" fn(
        ctx: *mut ::core::ffi::c_void,
        backup_info: *mut Backup_Info2,
        size_backup_info: u32,
        uploadcallbackend: fnNotifyUploadAsyncCompletion,
        uploadasynccompletionctx: *mut ::core::ffi::c_void,
    ) -> (),
>;

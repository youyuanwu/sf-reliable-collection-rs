use std::sync::Arc;

use mssf_com::{
    FabricCommon::{IFabricAsyncOperationCallback, IFabricAsyncOperationContext},
    FabricRuntime::{
        IFabricOperationData, IFabricOperationDataStream, IFabricOperationDataStream_Impl,
        IFabricOperationStream2,
    },
};
use mssf_core::{
    runtime::executor::Executor,
    sync::{fabric_begin_bridge, fabric_begin_end_proxy, fabric_end_bridge},
};
use windows_core::{implement, Interface};

use crate::{
    data::{OperationDataBridge, OperationDataProxy},
    operation::OperationProxy,
    traits::{Operation, OperationData, OperationDataStream, OperationStream},
};

// data stream bridge
#[implement(IFabricOperationDataStream)]
pub struct OpeartionDataStreamBridge<T, E>
where
    T: OperationDataStream,
    E: Executor,
{
    inner: Arc<T>,
    rt: E,
}

impl<T: OperationDataStream, E: Executor> OpeartionDataStreamBridge<T, E> {
    pub fn new(stream: T, rt: E) -> Self {
        Self {
            inner: Arc::new(stream),
            rt,
        }
    }
}

impl<T: OperationDataStream, E: Executor> IFabricOperationDataStream_Impl
    for OpeartionDataStreamBridge<T, E>
{
    fn BeginGetNext(
        &self,
        callback: Option<&IFabricAsyncOperationCallback>,
    ) -> windows_core::Result<IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        fabric_begin_bridge(&self.rt, callback, async move {
            inner.get_next().await.map(|opt| {
                opt.map_or(
                    unsafe { IFabricOperationData::from_raw(std::ptr::null_mut()) }, // convert end of stream of none
                    |x| IFabricOperationData::from(OperationDataBridge::new(x)),
                )
            })
        })
    }

    fn EndGetNext(
        &self,
        context: Option<&IFabricAsyncOperationContext>,
    ) -> windows_core::Result<IFabricOperationData> {
        fabric_end_bridge(context)
    }
}

pub struct OperationDataStreamProxy {
    com_impl: IFabricOperationDataStream,
}

impl OperationDataStreamProxy {
    pub fn new(com_impl: IFabricOperationDataStream) -> Self {
        Self { com_impl }
    }
}

impl OperationDataStream for OperationDataStreamProxy {
    async fn get_next(&self) -> mssf_core::Result<Option<impl OperationData>> {
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let res = fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginGetNext(callback)
            },
            move |ctx| unsafe { com2.EndGetNext(ctx) },
        ).await;
        match res {
            Ok(data) => {
                let proxy = OperationDataProxy::new(data)?;
                Ok(Some(proxy))
            }
            Err(e) => {
                if e == mssf_core::Error::empty() {
                    // special case of end of stream.
                    // nullptr is returned and windows-rs gives an empty error.
                    Ok(None)
                } else {
                    Err(e)
                }
            }
        }
    }
}

pub struct OperationStreamProxy {
    com_impl: IFabricOperationStream2,
}

impl OperationStreamProxy {
    pub fn new(com_impl: IFabricOperationStream2) -> Self {
        Self { com_impl }
    }
}

impl OperationStream for OperationStreamProxy {
    async fn get_operation(&self) -> mssf_core::Result<Option<impl Operation>> {
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let res = fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginGetOperation(callback)
            },
            move |ctx| unsafe { com2.EndGetOperation(ctx) },
        ).await;
        match res {
            Ok(op) => {
                let proxy = OperationProxy::new(op);
                Ok(Some(proxy))
            }
            Err(e) => {
                if e == mssf_core::Error::empty() {
                    // special case of end of stream.
                    // nullptr is returned and windows-rs gives an empty error.
                    Ok(None)
                } else {
                    Err(e)
                }
            }
        }
    }

    fn report_fault(&self) -> mssf_core::Result<()> {
        todo!()
        //unsafe { self.com_impl.ReportFault() }
    }
}

#[cfg(test)]
mod test {
    use std::{cell::Cell, sync::Mutex};

    use bytes::{Buf, Bytes};
    use mssf_com::FabricRuntime::IFabricOperationDataStream;
    use mssf_core::runtime::executor::DefaultExecutor;

    use crate::{
        data::OperationDataBuf,
        stream::OperationDataStreamProxy,
        traits::{OperationData, OperationDataStream},
    };

    use super::OpeartionDataStreamBridge;

    struct MyOperationDataStream {
        count: Mutex<Cell<u16>>,
    }

    // dummy stream returns data 2 times and then none
    impl OperationDataStream for MyOperationDataStream {
        async fn get_next(&self) -> mssf_core::Result<Option<impl OperationData>> {
            let mut c = self.count.lock().unwrap();
            if c.get() == 2 {
                return Ok(None);
            }

            let buf = Bytes::from(format!("value{}", c.get()));
            *c.get_mut() += 1;
            Ok(Some(OperationDataBuf::new(buf)))
        }
    }

    #[tokio::test]
    async fn test_data_stream() {
        // get handle
        let h = tokio::runtime::Handle::current();
        let rt = DefaultExecutor::new(h);
        let mystream = MyOperationDataStream {
            count: Mutex::new(Cell::new(0)),
        };

        // wrap in bridge and back in proxy
        let bridge: IFabricOperationDataStream =
            OpeartionDataStreamBridge::new(mystream, rt).into();
        let proxy = OperationDataStreamProxy::new(bridge);

        let d0 = proxy.get_next().await.unwrap().unwrap();
        assert_eq!(d0.chunk(), "value0".as_bytes());
        let d1 = proxy.get_next().await.unwrap().unwrap();
        assert_eq!(d1.chunk(), "value1".as_bytes());
        let d2 = proxy.get_next().await.unwrap();
        assert!(d2.is_none());
    }
}

#[cfg(test)]
mod test {
    use fabric_c::Microsoft::ServiceFabric::ReliableCollectionRuntime::{
        ReliableCollectionRuntime_Initialize, RELIABLECOLLECTION_API_VERSION,
    };

    #[test]
    fn test_linking() {
        let ok =
            unsafe { ReliableCollectionRuntime_Initialize(RELIABLECOLLECTION_API_VERSION as u16) };
        ok.expect("cannot init")
    }
}

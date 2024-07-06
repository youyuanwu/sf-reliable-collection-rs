use std::{cell::Cell, sync::Mutex};

use bytes::Bytes;
use mssf_ext::{
    data::OperationDataBuf,
    traits::{OperationData, OperationDataStream},
};

// return counting data until max number of times.
pub struct CountingOperationDataStream {
    count: Mutex<Cell<usize>>,
    max: usize,
    data: &'static str,
}

impl CountingOperationDataStream {
    pub fn new(max: usize, data: &'static str) -> Self {
        Self {
            count: Mutex::new(Cell::new(0)),
            max,
            data,
        }
    }
}

// dummy stream returns data 2 times and then none
impl OperationDataStream for CountingOperationDataStream {
    async fn get_next(&self) -> mssf_core::Result<Option<impl OperationData>> {
        let mut c = self.count.lock().unwrap();
        if c.get() == self.max {
            return Ok(None);
        }

        let buf = Bytes::from(format!("{}:{}", &self.data, c.get()));
        *c.get_mut() += 1;
        Ok(Some(OperationDataBuf::new(buf)))
    }
}

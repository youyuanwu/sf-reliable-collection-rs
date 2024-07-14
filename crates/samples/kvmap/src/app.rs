use std::{cell::Cell, io::ErrorKind, path::Path};

use tracing::info;

use crate::data::SingleDB;

pub struct KvApp {
    db: tokio::sync::Mutex<Cell<Option<SingleDB>>>,
    //sr: StateReplicatorProxy
}

impl KvApp {
    pub fn create() -> Self {
        Self {
            db: tokio::sync::Mutex::new(Cell::new(None)),
            //sr
        }
    }

    pub async fn open(&self, data_dir: &Path) -> std::io::Result<()> {
        // open db
        let db = SingleDB::create_or_attach(data_dir).await?;
        let prev = self.db.lock().await.replace(Some(db));
        assert!(prev.is_none());
        Ok(())
    }

    pub async fn close(&self) {
        // unset db
        let db = self.db.lock().await;
        db.replace(None);
    }

    pub async fn set_data(&self, sn: i64, data: String) -> std::io::Result<()> {
        let mut lk = self.db.lock().await;
        if let Some(db) = lk.get_mut() {
            // TODO: replicate.
            // let mut sn = 0_i64;
            // let operation_data = OperationDataBuf::new(Bytes::from(data.clone()));
            // let sn2 = self.sr.replicate(operation_data, &mut sn).await.inspect_err(|e|{
            //   error!("replicate failed {}", e);
            // })?;
            // assert_eq!(sn2, sn);
            db.set_entry(sn, data).await?;
            info!("Set data ok with sn {}", sn);
        } else {
            info!("db not initialized.");
            return Err(std::io::Error::from(ErrorKind::NotFound));
        }
        Ok(())
    }

    pub async fn get_data(&self) -> std::io::Result<(i64, String)> {
        let mut lk = self.db.lock().await;
        if let Some(db) = lk.get_mut() {
            db.get_entry().await
        } else {
            info!("db not initialized.");
            Err(std::io::Error::from(ErrorKind::NotFound))
        }
    }
}

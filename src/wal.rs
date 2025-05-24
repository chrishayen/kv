use crate::encode::encode_wal_entry;

use bincode::Encode;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{Arc, Mutex};
pub struct WAL {
    wal: Arc<Mutex<File>>,
}

impl WAL {
    pub fn new(path: &str) -> Self {
        let wal = Arc::new(Mutex::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .unwrap(),
        ));

        Self { wal: wal }
    }

    pub fn append<V: Serialize + Encode>(
        &self,
        key: &str,
        value: V,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = self.wal.lock().unwrap();
        let entry = encode_wal_entry(key, &value)?;
        file.write_all(&entry)?;
        file.flush()?;
        Ok(())
    }

    pub fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = self.wal.lock().unwrap();
        file.flush()?;
        Ok(())
    }
}

impl Drop for WAL {
    fn drop(&mut self) {
        self.flush().unwrap();
    }
}

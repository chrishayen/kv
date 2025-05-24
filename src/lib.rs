use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

pub struct Store {
    memtable: Arc<Mutex<BTreeMap<String, String>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            memtable: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    pub async fn set(&self, key: String, value: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut memtable = self.memtable.lock().unwrap();
        memtable.insert(key.clone(), value.clone());

        Ok(())
    }

    pub async fn get(&self, key: String) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let memtable = self.memtable.lock().unwrap();
        if let Some(value) = memtable.get(&key) {
            return Ok(Some(value.clone()));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {}
}

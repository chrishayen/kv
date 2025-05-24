use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use bincode::{self, Decode, Encode};
use serde::{Serialize, de::DeserializeOwned};

pub struct Store {
    memtable: Arc<RwLock<BTreeMap<String, Vec<u8>>>>,
    pub memtable_size_limit: usize,
    bincode_config: bincode::config::Configuration,
}

impl Store {
    pub fn new() -> Self {
        Self {
            memtable: Arc::new(RwLock::new(BTreeMap::new())),
            bincode_config: bincode::config::standard().with_little_endian(),
            memtable_size_limit: 10,
        }
    }

    pub fn set<V: Serialize + Encode>(
        &self,
        key: &str,
        value: V,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let serialized_value = bincode::encode_to_vec(&value, self.bincode_config)?;
        let mut memtable = self.memtable.write().unwrap();
        memtable.insert(key.to_string(), serialized_value);

        Ok(())
    }

    pub fn get<V: DeserializeOwned + Decode<()>>(
        &self,
        key: String,
    ) -> Result<Option<V>, Box<dyn std::error::Error>> {
        let memtable = self.memtable.read().unwrap();
        if let Some(serialized_value) = memtable.get(&key) {
            let (value, _): (V, _) =
                bincode::decode_from_slice(serialized_value, self.bincode_config)?;
            return Ok(Some(value));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let store = Store::new();
        store.set("key", "value".to_string()).unwrap();
        let value = store.get("key".to_string()).unwrap();
        assert_eq!(value, Some("value".to_string()));

        store.set("number", 10).unwrap();
        let value = store.get("number".to_string()).unwrap();
        assert_eq!(value, Some(10));
    }
}

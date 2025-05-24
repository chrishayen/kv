use bincode::{self, Decode, Encode};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

pub struct Memtable {
    memtable: Arc<RwLock<BTreeMap<String, Vec<u8>>>>,
    bincode_config: bincode::config::Configuration,
}

impl Memtable {
    pub fn new() -> Self {
        Self {
            memtable: Arc::new(RwLock::new(BTreeMap::new())),
            bincode_config: bincode::config::standard().with_little_endian(),
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
        key: &str,
    ) -> Result<Option<V>, Box<dyn std::error::Error>> {
        let memtable = self.memtable.read().unwrap();
        if let Some(serialized_value) = memtable.get(key) {
            let (value, _) = bincode::decode_from_slice(serialized_value, self.bincode_config)?;
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
        let store = Memtable::new();

        store.set("one", "value").unwrap();
        store.set("two", 10).unwrap();

        assert_eq!(store.get("one").unwrap(), Some("value".to_string()));
        assert_eq!(store.get("two").unwrap(), Some(10));
    }
}

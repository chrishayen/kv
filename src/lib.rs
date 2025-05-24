mod memtable;
use bincode::{self, Decode, Encode};
use memtable::Memtable;
use serde::{Serialize, de::DeserializeOwned};

pub struct Store {
    memtable: Memtable,
    pub memtable_size_limit: usize,
}

impl Store {
    pub fn new() -> Self {
        Self {
            memtable: Memtable::new(),
            memtable_size_limit: 10,
        }
    }

    pub fn set<V: Serialize + Encode>(
        &self,
        key: &str,
        value: V,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.memtable.set(key, value)
    }

    pub fn get<V: DeserializeOwned + Decode<()>>(
        &self,
        key: &str,
    ) -> Result<Option<V>, Box<dyn std::error::Error>> {
        self.memtable.get(key)
    }
}

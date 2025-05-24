mod encode;
mod memtable;
mod wal;

use bincode::{self, Decode, Encode};
use memtable::Memtable;
use serde::{Serialize, de::DeserializeOwned};
use wal::WAL;

pub struct Store {
    memtable: Memtable,
    wal: WAL,
    pub memtable_size_limit: usize,
}

impl Store {
    pub fn new() -> Self {
        Self {
            memtable: Memtable::new(),
            memtable_size_limit: 10,
            wal: WAL::new("./wal.log"),
        }
    }

    pub fn put<V: Serialize + Encode>(
        &self,
        key: &str,
        value: V,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.wal.append(key, &value)?;
        self.memtable.set(key, &value)?;

        Ok(())
    }

    pub fn get<V: DeserializeOwned + Decode<()>>(
        &self,
        key: &str,
    ) -> Result<Option<V>, Box<dyn std::error::Error>> {
        self.memtable.get(key)
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

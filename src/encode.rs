// fn encode<T: Serialize + Encode>(value: T) -> Vec<u8> {
//     return bincode::encode_to_vec(&value, self.bincode_config)?;
// }

use bincode::Encode;
use serde::Serialize;

pub fn get_config() -> bincode::config::Configuration {
    bincode::config::standard().with_little_endian()
}

pub fn encode_wal_entry<T: Serialize + Encode>(
    key: &str,
    value: &T,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(bincode::encode_to_vec(&(key, value), get_config())?)
}

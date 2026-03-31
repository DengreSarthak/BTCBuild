use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uint::construct_uint;
construct_uint! {
    pub struct U256(4);
}

impl Serialize for U256 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let bytes = self.to_big_endian();
        serializer.serialize_bytes(&bytes)
    }
}

impl<'de> Deserialize<'de> for U256 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let bytes: Vec<u8> = Vec::<u8>::deserialize(deserializer)?;
        if bytes.len() > 32 {
            return Err(serde::de::Error::custom("U256 expects at most 32 bytes"));
        }
        let mut buf = [0u8; 32];
        buf[32 - bytes.len()..].copy_from_slice(&bytes);
        Ok(U256::from_big_endian(&buf))
    }
}

pub mod error;
pub mod crypto;
pub mod sha256;
pub mod types;
pub mod util;
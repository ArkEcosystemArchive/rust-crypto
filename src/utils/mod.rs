pub mod slot;
pub mod message;

use failure;
use hex;

pub fn str_from_hex(string: &str) -> Result<String, failure::Error> {
    Ok(String::from_utf8(hex::decode(string)?)?.to_string())
}

pub fn str_to_hex(string: &str) -> String {
    hex::encode(string.as_bytes())
}

mod macros;
pub mod message;
pub mod slot;

use failure;
use hex;

pub use self::message::Message;

pub fn str_from_hex(string: &str) -> Result<String, failure::Error> {
    Ok(String::from_utf8(hex::decode(string)?))
}

pub fn str_to_hex(string: &str) -> String {
    hex::encode(string.as_bytes())
}

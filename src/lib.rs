extern crate bitcoin;
extern crate byteorder;
extern crate chrono;
extern crate failure;
extern crate hex;
#[macro_use]
extern crate lazy_static;
extern crate ripemd160;
extern crate secp256k1;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sha2;

#[macro_use]
pub mod utils;
pub mod configuration;
pub mod enums;
pub mod identities;
pub mod transactions;

use secp256k1::{All, Secp256k1};

lazy_static! {
    pub static ref SECP256K1: Secp256k1<All> = Secp256k1::new();
}

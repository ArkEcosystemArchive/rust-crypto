extern crate arkecosystem_crypto;
extern crate serde;
#[macro_use]
extern crate serde_json;

use serde_json::{from_str, Value};
use std::fs::File;
use std::io::prelude::*;

pub mod transactions;

pub fn json_transaction(transaction_type: &str, name: &str) -> Value {
    let path = read_fixture(&format!("transactions/{}/{}", transaction_type, name));
    from_str(&path).unwrap()
}

fn read_fixture(path: &str) -> String {
    let mut file = File::open(format!("tests/fixtures/{}.json", path)).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content
}

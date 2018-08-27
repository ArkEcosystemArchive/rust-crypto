use std::sync::Mutex;
use std::boxed::Box;
use networks::{Network, mainnet::Mainnet};

lazy_static! {
    static ref NETWORK: Mutex<Box<Network + Send>> = {
        Mutex::new(Box::new(Mainnet {}))
    };
}

pub fn set<T: Network + Send + 'static>(network: T) {
    *NETWORK.lock().unwrap() = Box::new(network);
}

pub fn epoch() -> &'static str {
    NETWORK.lock().unwrap().epoch()
}

pub fn version() -> u16 {
    NETWORK.lock().unwrap().version()
}

pub fn wif() -> u32 {
    NETWORK.lock().unwrap().wif()
}

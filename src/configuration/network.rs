use enums::networks::Network;
use std::sync::Mutex;

lazy_static! {
    static ref NETWORK: Mutex<Network> = Mutex::new(Network::Mainnet);
}

pub fn set(network: Network) {
    *NETWORK.lock().unwrap() = network;
}

pub fn get() -> Network {
    (*NETWORK.lock().unwrap()).clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_network() {
        assert_eq!(get(), Network::Mainnet);
    }

    #[test]
    fn set_network() {
        set(Network::Devnet);
        assert_eq!(get(), Network::Devnet);
        set(Network::Mainnet);
    }
}

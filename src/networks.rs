#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Network {
    Mainnet,
    Devnet,
    Testnet,
}

impl Network {
    pub fn epoch(&self) -> &'static str {
        match *self {
            Network::Mainnet => "2017-03-21T13:00:00.000Z",
            Network::Devnet => "2017-03-21T13:00:00.000Z",
            Network::Testnet => "2017-03-21T13:00:00.000Z",
        }
    }

    pub fn version(&self) -> u8 {
        match *self {
            Network::Mainnet => 0x17,
            Network::Devnet => 0x1e,
            Network::Testnet => 0x17,
        }
    }

    pub fn wif(&self) -> u8 {
        match *self {
            Network::Mainnet => 170,
            Network::Devnet => 170,
            Network::Testnet => 186,
        }
    }
}

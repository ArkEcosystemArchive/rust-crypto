use networks::Network;

pub struct Testnet;

impl Network for Testnet {

    fn epoch(&self) -> &'static str {
        "2017-03-21T13:00:00.000Z"
    }

    fn version(&self) -> u16 {
        0x17
    }

    fn wif(&self) -> u32 {
        186
    }

}

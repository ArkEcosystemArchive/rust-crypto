use networks::Network;

pub struct Mainnet;

impl Network for Mainnet {

    fn epoch() -> &'static str {
        "2017-03-21T13:00:00.000Z"
    }

    fn version() -> u16 {
        0x17
    }

    fn wif() -> u32 {
        170
    }

}

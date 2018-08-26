use networks::Network;

pub struct Devnet;

impl Network for Devnet {

    fn epoch() -> &'static str {
        "2017-03-21T13:00:00.000Z"
    }

    fn version() -> u16 {
        0x1e
    }

    fn wif() -> u32 {
        170
    }

}

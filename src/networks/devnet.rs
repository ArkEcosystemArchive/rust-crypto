use networks::Network;

pub struct Devnet;

impl Network for Devnet {

    fn epoch(&self) -> &'static str {
        "2017-03-21T13:00:00.000Z"
    }

    fn version(&self) -> u16 {
        0x1e
    }

    fn wif(&self) -> u32 {
        170
    }

}

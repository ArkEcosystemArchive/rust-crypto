pub mod devnet;
pub mod mainnet;
pub mod testnet;

pub trait Network {
    fn epoch(&self) -> &'static str;
    fn version(&self) -> u16;
    fn wif(&self) -> u32;
}

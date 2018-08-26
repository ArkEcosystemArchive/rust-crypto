
pub mod devnet;
pub mod mainnet;
pub mod testnet;

pub trait Network {
    fn epoch() -> &'static str;
    fn version() -> u16;
    fn wif() -> u32;
}

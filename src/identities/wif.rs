use sha2::{Digest, Sha256};
use bs58;
use std::str;

use super::super::configuration;

pub fn from_passphrase(passphrase: &str) -> String {

    let output = Sha256::digest_str(passphrase);

    let wif = configuration::network::get().wif();
    let mut seed = vec![wif];
    seed.extend_from_slice(&output);
    seed.push(0x01);

    // https://en.bitcoin.it/wiki/Base58Check_encoding
    let checked = Sha256::digest(Sha256::digest(seed.as_slice()).as_slice());
    seed.extend_from_slice(&checked[0..4]);

    bs58::encode(seed).into_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn wif_from_passphrase() {
        assert_eq!(from_passphrase("this is a top secret passphrase"), "SGq4xLgZKCGxs7bjmwnBrWcT4C1ADFEermj846KC97FSv1WFD1dA");
    }
}

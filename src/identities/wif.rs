use sha2::{Digest, Sha256};
use bitcoin::util::base58;

use super::super::configuration;

pub fn from_passphrase(passphrase: &str) -> String {
    let mut bytes = vec![];
    bytes.push(configuration::network::get().wif());
    bytes.extend_from_slice(&Sha256::digest_str(passphrase));
    bytes.push(0x01);

    base58::check_encode_slice(&bytes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn wif_from_passphrase() {
        assert_eq!(
            from_passphrase("this is a top secret passphrase"),
            "SGq4xLgZKCGxs7bjmwnBrWcT4C1ADFEermj846KC97FSv1WFD1dA"
        );
    }
}

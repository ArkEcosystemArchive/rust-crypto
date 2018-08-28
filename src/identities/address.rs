use secp256k1::{Error, PublicKey};
use ripemd160::{Digest, Ripemd160};
use bitcoin::util::base58;
use hex;

use super::public_key;
use super::private_key;
use super::private_key::PrivateKey;
use super::super::configuration;

pub fn from_passphrase(passphrase: &str) -> Result<String, Error> {
    let private_key = private_key::from_passphrase(passphrase)?;
    Ok(from_private_key(&private_key))
}

pub fn from_private_key(private_key: &PrivateKey) -> String {
    let public_key = public_key::from_private_key(private_key);
    from_public_key(&public_key)
}

pub fn from_public_key(public_key: &PublicKey) -> String {
    // TODO: fix unwrap
    let bytes = hex::decode(public_key.to_string()).unwrap();

    let ripemd160 = Ripemd160::digest(&bytes);
    let mut data = vec![];
    data.push(configuration::network::get().version());
    data.extend_from_slice(&ripemd160);
    base58::check_encode_slice(&data)
}

pub fn validate(address: &str) -> bool {
    let network = configuration::network::get().version();
    let bytes = base58::from_check(address);
    if bytes.is_ok() {
        return *bytes.unwrap().first().unwrap() == network;
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;
    use networks::Network;

    #[test]
    fn address_from_passphrase() {
        configuration::network::set(Network::Devnet);
        let private_key = from_passphrase("this is a top secret passphrase");
        assert_eq!(
            private_key.unwrap().to_string(),
            "D61mfSggzbvQgTUe6JhYKH2doHaqJ3Dyib"
        );

        configuration::network::set(Network::Mainnet);
    }

    #[test]
    fn private_key_from_hex() {
        //    let private_key = from_hex("d8839c2432bfd0a67ef10a804ba991eabba19f154a3d707917681d45822a5712");
        //        assert!(private_key.is_ok());
        //        assert_eq!(private_key.unwrap().to_string(), "d8839c2432bfd0a67ef10a804ba991eabba19f154a3d707917681d45822a5712");
    }
}

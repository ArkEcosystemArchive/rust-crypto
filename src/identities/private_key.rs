use secp256k1::{Secp256k1, Error, SecretKey};
use sha2::{Digest, Sha256};
use hex;

pub type PrivateKey = SecretKey;

pub fn from_passphrase(passphrase: &str) -> Result<PrivateKey, Error> {
    PrivateKey::from_slice(&Secp256k1::new(), &Sha256::digest_str(passphrase)[..])
}

pub fn from_hex(private_key: &str) -> Result<PrivateKey, Error> {
    // TODO: fix unwrap
    PrivateKey::from_slice(&Secp256k1::new(), hex::decode(private_key).unwrap().as_slice())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn private_key_from_passphrase() {
        let private_key = from_passphrase("this is a top secret passphrase");
        assert!(private_key.is_ok());
        assert_eq!(private_key.unwrap().to_string(), "d8839c2432bfd0a67ef10a804ba991eabba19f154a3d707917681d45822a5712");
    }

    #[test]
    fn private_key_from_hex() {
        let private_key = from_hex("d8839c2432bfd0a67ef10a804ba991eabba19f154a3d707917681d45822a5712");
        assert!(private_key.is_ok());
        assert_eq!(private_key.unwrap().to_string(), "d8839c2432bfd0a67ef10a804ba991eabba19f154a3d707917681d45822a5712");
    }
}

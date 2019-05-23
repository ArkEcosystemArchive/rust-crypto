use super::super::SECP256k1;
use hex;
use secp256k1::{Error, Message, SecretKey};
use sha2::{Digest, Sha256};

pub type PrivateKey = SecretKey;

pub fn from_passphrase(passphrase: &str) -> Result<PrivateKey, Error> {
    PrivateKey::from_slice(&Sha256::digest(passphrase.as_bytes())[..])
}

pub fn from_hex(private_key: &str) -> Result<PrivateKey, Error> {
    // TODO: fix unwrap
    PrivateKey::from_slice(hex::decode(private_key).unwrap().as_slice())
}

pub fn sign(bytes: &[u8], passphrase: &str) -> String {
    let key = from_passphrase(passphrase).unwrap();
    let hash = &Sha256::digest(&bytes);
    let msg = Message::from_slice(&hash).unwrap();
    let sig = SECP256K1.sign(&msg, &key);

    hex::encode(sig.serialize_der())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn private_key_from_passphrase() {
        let private_key = from_passphrase("this is a top secret passphrase");
        assert!(private_key.is_ok());
        assert_eq!(
            private_key.unwrap().to_string(),
            "d8839c2432bfd0a67ef10a804ba991eabba19f154a3d707917681d45822a5712"
        );
    }

    #[test]
    fn private_key_from_hex() {
        let private_key =
            from_hex("d8839c2432bfd0a67ef10a804ba991eabba19f154a3d707917681d45822a5712");
        assert!(private_key.is_ok());
        assert_eq!(
            private_key.unwrap().to_string(),
            "d8839c2432bfd0a67ef10a804ba991eabba19f154a3d707917681d45822a5712"
        );
    }
}

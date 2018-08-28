use secp256k1::{Error, PublicKey, Secp256k1};
use hex;

use super::private_key;

pub fn from_passphrase(passphrase: &str) -> Result<PublicKey, Error> {
    let private_key = private_key::from_passphrase(passphrase)?;
    Ok(PublicKey::from_secret_key(&Secp256k1::new(), &private_key))
}

pub fn from_hex(public_key: &str) -> Result<PublicKey, Error> {
    // TODO: fix unwrap
    PublicKey::from_slice(
        &Secp256k1::new(),
        hex::decode(public_key).unwrap().as_slice(),
    )
}

pub fn from_private_key(private_key: &private_key::PrivateKey) -> PublicKey {
    PublicKey::from_secret_key(&Secp256k1::new(), private_key)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn public_key_from_passphrase() {
        let public_key = from_passphrase("this is a top secret passphrase");
        assert!(public_key.is_ok());
        assert_eq!(
            public_key.unwrap().to_string(),
            "034151a3ec46b5670a682b0a63394f863587d1bc97483b1b6c70eb58e7f0aed192"
        );
    }

    #[test]
    fn public_key_from_hex() {
        let public_key =
            from_hex("034151a3ec46b5670a682b0a63394f863587d1bc97483b1b6c70eb58e7f0aed192");
        assert!(public_key.is_ok());
        assert_eq!(
            public_key.unwrap().to_string(),
            "034151a3ec46b5670a682b0a63394f863587d1bc97483b1b6c70eb58e7f0aed192"
        );
    }
}

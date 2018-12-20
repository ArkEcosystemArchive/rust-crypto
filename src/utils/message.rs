use hex;
use secp256k1::Signature;
use serde_json;
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::super::identities::{private_key, public_key};
use super::super::SECP256k1;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    #[serde(rename = "publickey")]
    pub public_key: String,
    pub signature: String,
    pub message: String,
}

impl Message {
    pub fn new(public_key: &str, signature: &str, message: &str) -> Message {
        Message {
            public_key: public_key.to_owned(),
            signature: signature.to_owned(),
            message: message.to_owned(),
        }
    }

    pub fn sign(message: &str, passphrase: &str) -> Message {
        let key = private_key::from_passphrase(passphrase).unwrap();
        let public_key = public_key::from_private_key(&key);

        Message {
            public_key: hex::encode(&public_key.serialize()[..]),
            signature: private_key::sign(message.as_bytes(), passphrase),
            message: message.to_owned(),
        }
    }

    // TODO: unwrap
    pub fn verify(&self) -> bool {
        let hash = Sha256::digest(&self.message.as_bytes());

        let message = secp256k1::Message::from_slice(&hash);
        if message.is_err() {
            return false;
        }

        let decoded = hex::decode(&self.signature);
        if decoded.is_err() {
            return false;
        }

        let signature = Signature::from_der(&decoded.unwrap());
        if signature.is_err() {
            return false;
        }

        let pk = public_key::from_hex(&self.public_key).unwrap();
        SECP256k1
            .verify(&message.unwrap(), &signature.unwrap(), &pk)
            .is_ok()
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn to_map(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify() {
        let m = Message::new(
                "034151a3ec46b5670a682b0a63394f863587d1bc97483b1b6c70eb58e7f0aed192",
                "304402200fb4adddd1f1d652b544ea6ab62828a0a65b712ed447e2538db0caebfa68929e02205ecb2e1c63b29879c2ecf1255db506d671c8b3fa6017f67cfd1bf07e6edd1cc8",
                "Hello World"
            );

        assert!(m.verify());
    }

    #[test]
    fn test_sign() {
        let sig = Message::sign("Hello World", "this is a top secret passphrase");
        assert_eq!(sig.signature, "304402200fb4adddd1f1d652b544ea6ab62828a0a65b712ed447e2538db0caebfa68929e02205ecb2e1c63b29879c2ecf1255db506d671c8b3fa6017f67cfd1bf07e6edd1cc8");
    }

}

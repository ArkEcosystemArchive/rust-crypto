use bitcoin::util::base58;
use byteorder::{LittleEndian, WriteBytesExt};
use hex;
use secp256k1;
use secp256k1::{Secp256k1, Signature};
use serde_json;
use sha2::{Digest, Sha256};
use std::iter;

use enums::types::Types;
use identities::{private_key, public_key};
use utils::message::Message;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[serde(skip)]
    pub header: u8,
    pub network: u8,
    #[serde(rename = "type")]
    pub type_id: Types,
    pub version: u8,
    pub asset: Asset,
    #[serde(skip)]
    pub timelock_type: u32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub signatures: Vec<String>,
    pub id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub recipient_id: String,
    #[serde(skip)]
    pub second_signature: String,
    pub sender_public_key: String,
    pub signature: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub sign_signature: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub vendor_field: String,
    #[serde(skip)]
    pub vendor_field_hex: String,
    #[serde(skip)]
    pub expiration: u32,
    pub timestamp: u32,
    pub amount: u64,
    pub fee: u64,
    #[serde(skip)]
    pub timelock: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Asset {
    #[serde(skip)]
    None,
    Signature {
        public_key: String,
    },
    Delegate {
        username: String,
    },
    Votes {
        votes: Vec<String>,
    },
    #[serde(rename = "multisignature")]
    MultiSignatureRegistration {
        min: u8,
        keysgroup: Vec<String>,
        lifetime: u8,
    },
}

impl Default for Asset {
    fn default() -> Asset {
        Asset::None
    }
}

impl Transaction {
    pub fn get_id(&self) -> String {
        let bytes = self.to_bytes(false, false);
        hex::encode(Sha256::digest(&bytes))
    }

    // TODO: unwrap
    pub fn sign(&mut self, passphrase: &str) -> &Self {
        let private_key = private_key::from_passphrase(passphrase).unwrap();
        let public_key = public_key::from_private_key(&private_key);
        self.sender_public_key = hex::encode(public_key.serialize().to_vec());

        let message = &hex::encode(Sha256::digest(&self.to_bytes(true, true)));
        self.signature = Message::sign(message, passphrase).signature;

        self
    }

    pub fn second_sign(&mut self, passphrase: &str) -> &Self {
        let message = &hex::encode(Sha256::digest(&self.to_bytes(false, true)));
        self.sign_signature = Message::sign(message, passphrase).signature;

        self
    }

    pub fn verify(&self) -> bool {
        self.internal_verify(&self.signature, &self.to_bytes(true, true))
    }

    pub fn second_verify(&self) -> bool {
        self.internal_verify(&self.sign_signature, &self.to_bytes(false, true))
    }

    pub fn to_bytes(&self, skip_signature: bool, skip_second_signature: bool) -> Vec<u8> {
        let mut buffer = vec![];

        buffer.write_u8(self.type_id.clone() as u8).unwrap();
        buffer.write_u32::<LittleEndian>(self.timestamp).unwrap();

        buffer.extend_from_slice(&hex::decode(&self.sender_public_key).unwrap());

        let recipient_id = if self.recipient_id.len() > 0 {
            base58::from_check(&self.recipient_id).unwrap()
        } else {
            iter::repeat(0).take(21).collect()
        };

        buffer.extend_from_slice(&recipient_id);

        let vendor_field: Vec<u8> = if self.vendor_field.len() > 0 {
            let vendor_bytes = self.vendor_field.as_bytes();
            if vendor_bytes.len() <= 64 {
                vendor_bytes
                    .iter()
                    .cloned()
                    .chain(
                        iter::repeat(0)
                            .take(64 - vendor_bytes.len())
                            .collect::<Vec<u8>>(),
                    )
                    .collect::<Vec<u8>>()
            } else {
                vendor_bytes.to_vec()
            }

        } else {
            iter::repeat(0).take(64).collect()
        };

        buffer.extend_from_slice(&vendor_field);
        buffer.write_u64::<LittleEndian>(self.amount).unwrap();
        buffer.write_u64::<LittleEndian>(self.fee).unwrap();

        // Payload
        let payload: Vec<u8> = match &self.asset {
            &Asset::Signature { ref public_key } => hex::decode(&public_key).unwrap(),
            &Asset::Delegate { ref username } => username.to_owned().as_bytes().to_vec(),
            &Asset::Votes { ref votes } => votes.join("").as_bytes().to_vec(),
            &Asset::MultiSignatureRegistration {
                min,
                lifetime,
                ref keysgroup,
            } => {
                let mut buffer = vec![];
                buffer.push(min);
                buffer.push(lifetime);
                buffer.extend_from_slice(keysgroup.clone().join("").as_bytes());

                buffer
            }
            _ => vec![],
        };

        buffer.extend_from_slice(&payload);

        // Signature
        if !skip_signature && self.signature.len() > 0 {
            buffer.extend_from_slice(&hex::decode(&self.signature).unwrap());
        }

        // Second Signature
        if !skip_second_signature && self.second_signature.len() > 0 {
            buffer.extend_from_slice(&hex::decode(&self.second_signature).unwrap());
        }

        buffer
    }

    fn internal_verify(&self, signature: &str, bytes: &[u8]) -> bool {
        let hash = Sha256::digest(&bytes);
        let msg = secp256k1::Message::from_slice(&hash).unwrap();

        let secp = Secp256k1::new();
        let sig = Signature::from_der(&secp, signature.as_bytes()).unwrap();
        let pk = public_key::from_hex(&self.sender_public_key).unwrap();

        secp.verify(&msg, &sig, &pk).is_ok()
    }

    pub fn to_params(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_to_bytes() {
        let mut transaction = Transaction::default();
        transaction.type_id = Types::Vote;
        transaction.timestamp = 39999;
        transaction.sign("this is a top secret passphrase");

        println!("{:?}", transaction.to_bytes(true, true));
    }

    #[test]
    fn test_aaaa() {
        let mut transaction = Transaction::default();
        transaction.sign("this is a top secret passphrase");

        println!("{:?}", transaction);
    }

}

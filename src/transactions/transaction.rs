use hex;
use byteorder::{LittleEndian, WriteBytesExt};
use bitcoin::util::base58;
use sha2::{Digest, Sha256};
use std::iter;

use enums::types::Types;
use identities::{public_key, private_key};
use utils::message::Message;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub header: u8,
    pub network: u8,
    pub type_id: Types,
    pub version: u8,
    pub asset: Asset,
    pub timelock_type: u32,
    pub signatures: Vec<String>,
    pub id: String,
    pub recipient_id: String,
    pub second_signature: String,
    pub sender_public_key: String,
    pub signature: String,
    pub sign_signature: String,
    pub vendor_field: String,
    pub vendor_field_hex: String,
    pub expiration: u32,
    pub timestamp: u32,
    pub amount: u64,
    pub fee: u64,
    pub timelock: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Asset {
    signature: SecondSignatureRegistrationAsset,
    delegate: DelegateRegistrationAsset,
    votes: Vec<String>,
    multisignature: MultiSignatureRegistrationAsset,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SecondSignatureRegistrationAsset {
    public_key: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DelegateRegistrationAsset {
    username: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MultiSignatureRegistrationAsset {
    min: u8,
    keysgroup: Vec<String>,
    lifetime: u8,
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

    pub fn to_bytes(&self, skip_signature: bool, skip_second_signature: bool) -> Vec<u8> {
        let mut buffer = vec![];

        buffer.write_u8(self.type_id.clone() as u8).unwrap();
        buffer.write_u32::<LittleEndian>(self.timestamp).unwrap();

        buffer.extend_from_slice(self.sender_public_key.as_bytes());

        let recipient_id = if !self.recipient_id.is_empty() {
            base58::from_check(&self.recipient_id).unwrap()
        } else {
            iter::repeat(0).take(21).collect()
        };

        buffer.extend_from_slice(&recipient_id);

        let vendor_field: Vec<u8> = if !self.vendor_field.is_empty() {
            let vendor_bytes = self.vendor_field.as_bytes();
            if vendor_bytes.len() < 64 {
                return vendor_bytes
                    .iter()
                    .cloned()
                    .chain(
                        iter::repeat(0)
                            .take(64 - vendor_bytes.len())
                            .collect::<Vec<u8>>(),
                    )
                    .collect();
            }

            vendor_bytes.to_vec()
        } else {
            iter::repeat(0).take(64).collect()
        };

        buffer.extend_from_slice(&vendor_field);
        buffer.write_u64::<LittleEndian>(self.amount).unwrap();
        buffer.write_u64::<LittleEndian>(self.fee).unwrap();

        // Payload
        let payload: Vec<u8> = match self.type_id {
            Types::SecondSignatureRegistration => {
                hex::decode(&self.asset.signature.public_key).unwrap()
            }
            Types::DelegateRegistration => self.asset.delegate.username.as_bytes().to_vec(),
            Types::Vote => self.asset.votes.join("").as_bytes().to_vec(),
            Types::MultiSignatureRegistration => {
                let ms_asset = &self.asset.multisignature;
                let mut buffer = vec![];
                buffer.push(ms_asset.min);
                buffer.push(ms_asset.lifetime);
                buffer.extend_from_slice(ms_asset.keysgroup.join("").as_bytes());

                buffer
            }

            _ => vec![],
        };
        buffer.extend_from_slice(&payload);

        // Signature
        let signature = if !skip_signature && !self.signature.is_empty() {
            hex::decode(&self.signature).unwrap()
        } else {
            vec![]
        };

        // Second Signature
        let second_signature = if !skip_second_signature && !self.second_signature.is_empty() {
            hex::decode(&self.second_signature).unwrap()
        } else {
            vec![]
        };

        buffer.extend_from_slice(&signature);
        buffer.extend_from_slice(&second_signature);

        buffer
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
        println!("{:?}", transaction.to_bytes(true, true));
    }

}

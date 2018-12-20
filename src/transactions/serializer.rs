use bitcoin::util::base58;
use byteorder::{LittleEndian, WriteBytesExt};
use hex;
use std::io::prelude::*;

use configuration::network;
use enums::TransactionType;
use transactions::transaction::{Asset, Transaction};

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn serialize(transaction: &Transaction) -> String {
    let mut bytes = vec![];
    bytes.write_u8(0xff).unwrap();
    bytes.write_u8(if transaction.version > 0 { transaction.version } else { 0x01 }).unwrap();
    bytes.write_u8(if transaction.network > 0 { transaction.network } else { network::get().version() }).unwrap();
    bytes.write_u8(transaction.type_id as u8).unwrap();
    bytes.write_u32::<LittleEndian>(transaction.timestamp).unwrap();
    bytes.write_all(&hex::decode(&transaction.sender_public_key).unwrap()).unwrap();
    bytes.write_u64::<LittleEndian>(transaction.fee).unwrap();

    serialize_vendor_field(transaction, &mut bytes);
    serialize_type(transaction, &mut bytes);
    serialize_signatures(transaction, &mut bytes);

    hex::encode(bytes)
}

fn serialize_vendor_field(transaction: &Transaction, bytes: &mut Vec<u8>) {
    if !transaction.vendor_field.is_empty() {
        let vendor_field_length = transaction.vendor_field.len() as u8;
        bytes.write_u8(vendor_field_length).unwrap();
        bytes
            .write_all(transaction.vendor_field.as_bytes())
            .unwrap();
    } else if !transaction.vendor_field_hex.is_empty() {
        let vendor_field_hex_length = transaction.vendor_field_hex.len() / 2;
        bytes.write_u8(vendor_field_hex_length as u8).unwrap();
        bytes
            .write_all(transaction.vendor_field_hex.as_bytes())
            .unwrap();
    } else {
        bytes.write_u8(0x00).unwrap();
    }
}

fn serialize_type(transaction: &Transaction, mut bytes: &mut Vec<u8>) {
    match transaction.type_id {
        TransactionType::Transfer => serialize_transfer(transaction, &mut bytes),
        TransactionType::SecondSignatureRegistration => {
            serialize_second_signature_registration(transaction, &mut bytes)
        }
        TransactionType::DelegateRegistration => {
            serialize_delegate_registration(transaction, &mut bytes)
        }
        TransactionType::Vote => serialize_vote(transaction, &mut bytes),
        TransactionType::MultiSignatureRegistration => {
            serialize_multi_signature_registration(transaction, &mut bytes)
        }
        TransactionType::Ipfs => (),
        TransactionType::TimelockTransfer => (),
        TransactionType::MultiPayment => (),
        TransactionType::DelegateResignation => (),
    }
}

fn serialize_transfer(transaction: &Transaction, bytes: &mut Vec<u8>) {
    bytes.write_u64::<LittleEndian>(transaction.amount).unwrap();
    bytes
        .write_u32::<LittleEndian>(transaction.expiration)
        .unwrap();

    let recipient_id = base58::from_check(&transaction.recipient_id).unwrap();
    bytes.write_all(&recipient_id).unwrap();
}

fn serialize_second_signature_registration(transaction: &Transaction, bytes: &mut Vec<u8>) {
    if let Asset::Signature { public_key } = &transaction.asset {
        let public_key_bytes = hex::decode(public_key).unwrap();
        bytes.write_all(&public_key_bytes).unwrap();
    }
}

fn serialize_delegate_registration(transaction: &Transaction, bytes: &mut Vec<u8>) {
    if let Asset::Delegate { username } = &transaction.asset {
        bytes.write_u8(username.len() as u8).unwrap();
        bytes.write_all(&username.as_bytes()).unwrap();
    }
}

fn serialize_vote(transaction: &Transaction, bytes: &mut Vec<u8>) {
    if let Asset::Votes(votes) = &transaction.asset {
        let mut vote_bytes = vec![];

        for vote in votes {
            let prefix = if vote.starts_with('+') { "01" } else { "00" };
            let _vote: String = vote.chars().skip(1).collect();
            vote_bytes.push(format!("{}{}", prefix, _vote));
        }

        bytes.write_u8(votes.len() as u8).unwrap();
        bytes
            .write_all(&hex::decode(&vote_bytes.join("")).unwrap())
            .unwrap();
    }
}

fn serialize_multi_signature_registration(transaction: &Transaction, bytes: &mut Vec<u8>) {
    if let Asset::MultiSignatureRegistration {
        min,
        keysgroup,
        lifetime,
    } = &transaction.asset
    {
        let keysgroup_string: String = keysgroup
            .iter()
            .map(|key| {
                if key.starts_with('+') {
                    key.chars().skip(1).collect::<String>()
                } else {
                    key.to_owned()
                }
            })
            .collect();

        bytes.write_u8(*min).unwrap();
        bytes.write_u8(keysgroup.len() as u8).unwrap();
        bytes.write_u8(*lifetime).unwrap();

        bytes
            .write_all(&hex::decode(keysgroup_string).unwrap())
            .unwrap();
    }
}

fn serialize_signatures(transaction: &Transaction, bytes: &mut Vec<u8>) {
    if !transaction.signature.is_empty() {
        write_decoded_hex(&transaction.signature, bytes);
    }

    if !transaction.second_signature.is_empty() {
        write_decoded_hex(&transaction.second_signature, bytes);
    } else if !transaction.sign_signature.is_empty() {
        write_decoded_hex(&transaction.sign_signature, bytes);
    }

    if !transaction.signatures.is_empty() {
        bytes.write_u8(0xff).unwrap();
        write_decoded_hex(&transaction.signatures.join(""), bytes);
    }
}

fn write_decoded_hex(signature: &str, bytes: &mut Vec<u8>) {
    let signatures_bytes = hex::decode(&signature).unwrap();
    bytes.write_all(&signatures_bytes).unwrap();
}

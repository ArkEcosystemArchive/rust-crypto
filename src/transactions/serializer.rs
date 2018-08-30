use bitcoin::util::base58;
use byteorder::{LittleEndian, WriteBytesExt};
use hex;
use std::io::prelude::*;

use configuration::network;
use enums::types::Types;
use transactions::transaction::{Asset, Transaction};

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn serialize(transaction: &Transaction) -> String {
    let mut bytes = vec![];
    bytes.write_u8(0xff).unwrap();
    bytes.write_u8(if transaction.version > 0 { transaction.version } else { 0x01 }).unwrap();
    bytes.write_u8(if transaction.network > 0 { transaction.network } else { network::get().version() }).unwrap();
    bytes.write_u8(transaction.type_id as u8).unwrap();
    bytes.write_u32::<LittleEndian>(transaction.timestamp).unwrap();
    bytes.write(transaction.sender_public_key.as_bytes()).unwrap();
    bytes.write_u64::<LittleEndian>(transaction.fee).unwrap();

    serialize_vendor_field(transaction, &mut bytes);
    serialize_type(transaction, &mut bytes);
    serialize_signatures(transaction, &mut bytes);

    hex::encode(bytes)
}

fn serialize_vendor_field(transaction: &Transaction, bytes: &mut Vec<u8>) {
    if transaction.vendor_field.len() > 0 {
        let vendor_field_length = transaction.vendor_field.len() as u8;
        bytes.write_u8(vendor_field_length).unwrap();
        bytes.write(transaction.vendor_field.as_bytes()).unwrap();
    } else if transaction.vendor_field_hex.len() > 0 {
        let vendor_field_hex_length = transaction.vendor_field_hex.len() / 2;
        bytes.write_u8(vendor_field_hex_length as u8).unwrap();
        bytes
            .write(transaction.vendor_field_hex.as_bytes())
            .unwrap();
    } else {
        bytes.write_u8(0x00).unwrap();
    }
}

fn serialize_type(transaction: &Transaction, mut bytes: &mut Vec<u8>) {
    match transaction.type_id {
        Types::Transfer => serialize_transfer(transaction, &mut bytes),
        Types::SecondSignatureRegistration => {
            serialize_second_signature_registration(transaction, &mut bytes)
        }
        Types::DelegateRegistration => serialize_delegate_registration(transaction, &mut bytes),
        Types::Vote => serialize_vote(transaction, &mut bytes),
        Types::MultiSignatureRegistration => {
            serialize_multi_signature_registration(transaction, &mut bytes)
        }
        Types::Ipfs => (),
        Types::TimelockTransfer => (),
        Types::MultiPayment => (),
        Types::DelegateResignation => (),
    }
}

fn serialize_transfer(transaction: &Transaction, bytes: &mut Vec<u8>) {
    bytes.write_u64::<LittleEndian>(transaction.amount).unwrap();
    bytes
        .write_u32::<LittleEndian>(transaction.expiration)
        .unwrap();

    let recipient_id = base58::from_check(&transaction.recipient_id).unwrap();
    let recipient_bytes = hex::encode(recipient_id);
    bytes.write(&recipient_bytes.as_bytes()).unwrap();
}

fn serialize_second_signature_registration(transaction: &Transaction, bytes: &mut Vec<u8>) {
    match &transaction.asset {
        Asset::Signature { public_key } => {
            let public_key_bytes = hex::decode(public_key).unwrap();
            bytes.write(&public_key_bytes).unwrap();
        }
        _ => (),
    }
}

fn serialize_delegate_registration(transaction: &Transaction, bytes: &mut Vec<u8>) {
    match &transaction.asset {
        Asset::Delegate { username } => {
            let username_length = username.len() / 2;
            bytes.write_u8(username_length as u8).unwrap();

            let username_bytes = hex::decode(username).unwrap();
            bytes.write(&username_bytes).unwrap();
        }
        _ => (),
    }
}

fn serialize_vote(transaction: &Transaction, bytes: &mut Vec<u8>) {
    match &transaction.asset {
        Asset::Votes(votes) => {
            let mut vote_bytes = vec![];

            for vote in votes {
                let prefix = if vote.starts_with("+") { "01" } else { "00" };
                let _vote: String = vote.chars().skip(1).collect();
                vote_bytes.push(format!("{}{}", prefix, _vote));
            }

            bytes.write_u8(votes.len() as u8).unwrap();
            bytes
                .write(&hex::decode(&vote_bytes.join("")).unwrap())
                .unwrap();
        }
        _ => (),
    }
}

fn serialize_multi_signature_registration(transaction: &Transaction, bytes: &mut Vec<u8>) {
    match &transaction.asset {
        Asset::MultiSignatureRegistration {
            min,
            keysgroup,
            lifetime,
        } => {
            let keysgroup_string: String = keysgroup
                .iter()
                .map(|key| {
                    if key.starts_with("+") {
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
                .write(&hex::decode(keysgroup_string).unwrap())
                .unwrap();
        }
        _ => (),
    }
}

fn serialize_signatures(transaction: &Transaction, bytes: &mut Vec<u8>) {
    if transaction.signature.len() > 0 {
        let signature_bytes = hex::decode(&transaction.signature).unwrap();
        bytes.write(&signature_bytes).unwrap();
    }

    if transaction.second_signature.len() > 0 {
        let second_signature_bytes = hex::decode(&transaction.second_signature).unwrap();
        bytes.write(&second_signature_bytes).unwrap();
    } else if transaction.sign_signature.len() > 0 {
        let sign_signature_bytes = hex::decode(&transaction.sign_signature).unwrap();
        bytes.write(&sign_signature_bytes).unwrap();
    }

    if transaction.signatures.len() > 0 {
        bytes.write_u8(0xff).unwrap();
        let signatures_bytes = hex::decode(&transaction.signatures.join("")).unwrap();
        bytes.write(&signatures_bytes).unwrap();
    }
}

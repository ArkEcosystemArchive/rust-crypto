use bitcoin::util::base58;
use byteorder::{LittleEndian, ReadBytesExt};
use hex;
use std::io::prelude::*;
use std::io::Cursor;
use std::io::SeekFrom;

use enums::TransactionType;
use identities::{address, public_key};
use transactions::transaction::{Asset, Transaction};
use utils;

pub fn deserialize(serialized: &str) -> Transaction {
    let decoded = hex::decode(serialized).unwrap();
    let mut bytes = Cursor::new(decoded.as_slice());
    let mut transaction = Transaction::default();

    let mut asset_offset = deserialize_header(&mut bytes, &mut transaction);
    deserialize_type(&mut bytes, &mut transaction, &serialized, &mut asset_offset);
    parse_signatures(&mut transaction, &serialized, asset_offset);

    if transaction.version == 1 {
        handle_version_one(&mut transaction);
    }

    transaction
}

fn deserialize_header(bytes: &mut Cursor<&[u8]>, transaction: &mut Transaction) -> usize {
    transaction.header = bytes.read_u8().unwrap();
    transaction.version = bytes.read_u8().unwrap();
    transaction.network = bytes.read_u8().unwrap();
    transaction.type_id = bytes.read_u8().unwrap().into();
    transaction.timestamp = bytes.read_u32::<LittleEndian>().unwrap();

    let mut sender_public_key_buf = [0; 33];
    bytes.read(&mut sender_public_key_buf).unwrap();
    transaction.sender_public_key = hex::encode(sender_public_key_buf.to_vec());
    transaction.fee = bytes.read_u64::<LittleEndian>().unwrap();

    let vendor_field_length = bytes.read_u8().unwrap() as usize;
    if vendor_field_length > 0 {
        let mut vendor_field_buf: Vec<u8> = vec![0; vendor_field_length];
        bytes.read(&mut vendor_field_buf).unwrap();
        transaction.vendor_field_hex = hex::encode(&vendor_field_buf);
    }

    (50 * 2 + vendor_field_length * 2) as usize
}

fn deserialize_type(
    bytes: &mut Cursor<&[u8]>,
    mut transaction: &mut Transaction,
    serialized: &str,
    mut asset_offset: &mut usize,
) {
    let type_id = transaction.type_id.clone();
    match type_id {
        TransactionType::Transfer => {
            deserialize_transfer(bytes, &mut transaction, &mut asset_offset)
        }
        TransactionType::SecondSignatureRegistration => deserialize_second_signature_registration(
            bytes,
            &mut transaction,
            serialized,
            &mut asset_offset,
        ),
        TransactionType::DelegateRegistration => deserialize_delegate_registration(
            bytes,
            &mut transaction,
            serialized,
            &mut asset_offset,
        ),
        TransactionType::Vote => {
            deserialize_vote(bytes, &mut transaction, serialized, &mut asset_offset)
        }
        TransactionType::MultiSignatureRegistration => {
            deserialize_multi_signature_registration(bytes, &mut transaction, &mut asset_offset)
        }
        TransactionType::Ipfs => (),
        TransactionType::TimelockTransfer => (),
        TransactionType::MultiPayment => (),
        TransactionType::DelegateResignation => (),
    }
}

fn deserialize_transfer(
    bytes: &mut Cursor<&[u8]>,
    transaction: &mut Transaction,
    asset_offset: &mut usize,
) {
    bytes
        .seek(SeekFrom::Start(*asset_offset as u64 / 2))
        .unwrap();

    transaction.amount = bytes.read_u64::<LittleEndian>().unwrap();
    transaction.expiration = bytes.read_u32::<LittleEndian>().unwrap();

    let mut recipient_id_buf = [0; 21];
    bytes.read(&mut recipient_id_buf).unwrap();
    transaction.recipient_id = base58::check_encode_slice(&recipient_id_buf);

    *asset_offset += (21 + 12) * 2;
}

fn deserialize_second_signature_registration(
    _bytes: &mut Cursor<&[u8]>,
    transaction: &mut Transaction,
    serialized: &str,
    asset_offset: &mut usize,
) {
    transaction.asset = Asset::Signature {
        public_key: serialized.chars().skip(*asset_offset).take(66).collect(),
    };

    *asset_offset += 66;
}

fn deserialize_delegate_registration(
    bytes: &mut Cursor<&[u8]>,
    transaction: &mut Transaction,
    serialized: &str,
    asset_offset: &mut usize,
) {
    let username_length = bytes.read_u8().unwrap() as usize;
    let username: String = serialized
        .chars()
        .skip(*asset_offset + 2)
        .take(username_length * 2)
        .collect();

    transaction.asset = Asset::Delegate {
        username: utils::str_from_hex(&username).unwrap(),
    };
    *asset_offset += (username_length + 1) * 2;
}

fn deserialize_vote(
    bytes: &mut Cursor<&[u8]>,
    transaction: &mut Transaction,
    serialized: &str,
    asset_offset: &mut usize,
) {
    let vote_length = bytes.read_u8().unwrap() as usize;

    *asset_offset += 2;

    let mut votes = Vec::with_capacity(vote_length);
    for i in 0..vote_length {
        let index_start = *asset_offset + (i * 2 * 34);
        let index_end = 2 * 34 - 2;

        let vote_type: String = serialized.chars().skip(index_start + 1).take(1).collect();

        let mut vote: String = serialized
            .chars()
            .skip(index_start + 2)
            .take(index_end)
            .collect();

        assert!(vote_type == "1" || vote_type == "0");
        if vote_type == "1" {
            vote.insert_str(0, "+");
        } else {
            vote.insert_str(0, "-");
        }

        votes.push(vote);
    }

    transaction.asset = Asset::Votes(votes);
    *asset_offset += vote_length * 34 * 2;
}

fn deserialize_multi_signature_registration(
    bytes: &mut Cursor<&[u8]>,
    transaction: &mut Transaction,
    asset_offset: &mut usize,
) {
    let min = bytes.read_u8().unwrap();
    let number_of_signatures = bytes.read_u8().unwrap() as usize;
    let lifetime = bytes.read_u8().unwrap();

    let mut keysgroup = Vec::with_capacity(number_of_signatures);
    for _ in 0..number_of_signatures {
        let mut public_key_buf = [0; 33];
        bytes.read(&mut public_key_buf).unwrap();
        keysgroup.push(hex::encode(public_key_buf.to_vec()))
    }

    transaction.asset = Asset::MultiSignatureRegistration {
        keysgroup,
        min,
        lifetime,
    };

    *asset_offset += 6 + number_of_signatures * 66;
}

fn parse_signatures(transaction: &mut Transaction, serialized: &str, asset_offset: usize) {
    let signature: String = serialized.chars().skip(asset_offset).collect();
    let mut multi_signature_offset = 0;

    if signature.len() > 0 {
        let signature_length_str: String = signature.chars().skip(2).take(2).collect();
        let signature_length =
            (u8::from_str_radix(&signature_length_str, 16).unwrap() + 2) as usize;

        transaction.signature = serialized
            .chars()
            .skip(asset_offset)
            .take(signature_length * 2)
            .collect();

        multi_signature_offset += signature_length * 2;

        let second_signature: String = serialized
            .chars()
            .skip(asset_offset + signature_length * 2)
            .collect();

        if second_signature.len() > 0 && !second_signature.starts_with("ff") {
            let second_signature_length_str: String =
                second_signature.chars().skip(2).take(2).collect();
            let second_signature_length =
                (u8::from_str_radix(&second_signature_length_str, 16).unwrap() + 2) as usize;

            transaction.second_signature = second_signature
                .chars()
                .take(second_signature_length * 2)
                .collect();

            multi_signature_offset += second_signature_length * 2;
        }

        let mut signatures: String = serialized
            .chars()
            .skip(asset_offset + multi_signature_offset)
            .collect();

        if signatures.is_empty() || !signatures.starts_with("ff") {
            return;
        }

        signatures = signatures.chars().skip(2).collect();

        loop {
            if signatures.is_empty() {
                break;
            }

            let multi_signature_length_str: String = signatures.chars().skip(2).take(2).collect();
            let multi_signature_length =
                (u8::from_str_radix(&multi_signature_length_str, 16).unwrap() + 2) as usize;

            if multi_signature_length > 0 {
                let multi_signature: String = signatures
                    .chars()
                    .take(multi_signature_length * 2)
                    .collect();

                transaction.signatures.push(multi_signature);

                signatures = signatures
                    .chars()
                    .skip(multi_signature_length * 2)
                    .collect();
            } else {
                break;
            }
        }
    }
}

fn handle_version_one(transaction: &mut Transaction) {
    if transaction.second_signature.len() > 0 {
        transaction.sign_signature = transaction.second_signature.to_owned();
    }

    match transaction.type_id {
        TransactionType::Vote => {
            // TODO: transaction.network
            let public_key = public_key::from_hex(&transaction.sender_public_key).unwrap();
            transaction.recipient_id = address::from_public_key(&public_key);
        }
        TransactionType::MultiSignatureRegistration => match &mut transaction.asset {
            &mut Asset::MultiSignatureRegistration {
                min: _,
                lifetime: _,
                ref mut keysgroup,
            } => {
                let mut keysgroup = keysgroup.as_mut_slice();
                for key in keysgroup {
                    *key = String::from("+") + key;
                }
            }
            _ => (),
        },
        _ => (),
    }

    if transaction.vendor_field_hex.len() > 0 {
        transaction.vendor_field = utils::str_from_hex(&transaction.vendor_field_hex).unwrap();
    }

    if transaction.id.is_empty() {
        transaction.id = transaction.get_id();
    }

    match transaction.type_id {
        TransactionType::SecondSignatureRegistration => {
            // TODO: transaction.network
            let public_key = public_key::from_hex(&transaction.sender_public_key).unwrap();
            transaction.recipient_id = address::from_public_key(&public_key);
        }
        TransactionType::MultiSignatureRegistration => {
            // TODO: transaction.network
            let public_key = public_key::from_hex(&transaction.sender_public_key).unwrap();
            transaction.recipient_id = address::from_public_key(&public_key);
        }
        _ => (),
    }
}

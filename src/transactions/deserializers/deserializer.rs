use hex;
use transactions::transaction::Transaction;
use enums::types::Types;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::prelude::*;

pub fn deserialize(serialized: &str) -> Transaction {
    let decoded = hex::decode(serialized).unwrap();
    let mut bytes = decoded.as_slice();
    let mut transaction = Transaction::default();

    let asset_offset = deserialize_header(&mut bytes, &mut transaction);

    transaction
}

fn deserialize_header(mut bytes: &[u8], transaction: &mut Transaction) -> u8 {
    bytes.read_u8().unwrap();
    transaction.version = bytes.read_u8().unwrap();
    transaction.network = bytes.read_u8().unwrap();
    transaction.type_id = bytes.read_u8().unwrap().into();
    transaction.timestamp = bytes.read_u32::<LittleEndian>().unwrap();

    let mut sender_public_key_buf = [0; 33];
    bytes.read(&mut sender_public_key_buf).unwrap();
    transaction.sender_public_key = hex::encode(sender_public_key_buf.to_vec());
    transaction.fee = bytes.read_u64::<LittleEndian>().unwrap();

    let vendor_field_length = bytes.read_u8().unwrap();
    if vendor_field_length > 0 {
        let mut vendor_field_buf = Vec::<u8>::with_capacity(vendor_field_length as usize);
        bytes.read(&mut vendor_field_buf).unwrap();

        transaction.vendor_field_hex = hex::encode(&vendor_field_buf);
    }

    50 * 2 + vendor_field_length * 2
}

fn deserialize_type(mut bytes: &[u8], transaction: &mut Transaction) {

    match transaction.type_id {
        Types::Transfer => (),
        Types::SecondSignatureRegistration => (),
        Types::DelegateRegistration => (),
        Types::Vote => (),
        Types::MultiSignatureRegistration => (),
        Types::Ipfs => (),
        Types::TimelockTransfer => (),
        Types::MultiPayment => (),
        Types::DelegateResignation => ()
    }

}

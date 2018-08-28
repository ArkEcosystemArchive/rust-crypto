use hex;
use transactions::transaction::Transaction;
use enums::types::Types;
use byteorder::{LittleEndian, ReadBytesExt};
use bitcoin::util::base58;
use std::io::Cursor;
use std::io::prelude::*;

pub fn deserialize(serialized: &str) -> Transaction {
    let decoded = hex::decode(serialized).unwrap();
    let mut bytes = Cursor::new(decoded.as_slice());
    let mut transaction = Transaction::default();

    let asset_offset = deserialize_header(&mut bytes, &mut transaction);
    deserialize_type(&mut bytes, &mut transaction, asset_offset);

    transaction
}

fn deserialize_header(bytes: &mut Cursor<&[u8]>, transaction: &mut Transaction) -> u8 {
    transaction.header = bytes.read_u8().unwrap();
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

fn deserialize_type(bytes: &mut Cursor<&[u8]>, mut transaction: &mut Transaction, asset_offset: u8) {

    let type_id = transaction.type_id.clone();
    match type_id {
        Types::Transfer => deserialize_vote(bytes, &mut transaction),
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

fn deserialize_vote(bytes: &mut Cursor<&[u8]>, transaction: &mut Transaction) {
    transaction.amount = bytes.read_u64::<LittleEndian>().unwrap();
    transaction.expiration = bytes.read_u32::<LittleEndian>().unwrap();

    let mut recipient_id_buf = [0; 21];
    bytes.read(&mut recipient_id_buf).unwrap();
    transaction.recipient_id = base58::check_encode_slice(&recipient_id_buf);
}

use arkecosystem_crypto::configuration::network;
use arkecosystem_crypto::enums::types::Types;
use arkecosystem_crypto::networks::Network;
use arkecosystem_crypto::transactions::deserializer;
use *;

#[test]
fn test_signed_with_a_passphrase() {
    let fixture = json_transaction("transfer", "passphrase");
    let serialized = &fixture["serialized"].as_str().unwrap();
    let transaction = deserializer::deserialize(serialized);

    network::set(Network::Devnet);

    assert_eq!(transaction.version, 1);
    assert_eq!(transaction.network, 30);
    assert_eq!(transaction.type_id, Types::Transfer);
    assert_eq!(
        transaction.timestamp,
        fixture["data"]["timestamp"].as_u64().unwrap() as u32
    );
    assert_eq!(
        transaction.sender_public_key,
        fixture["data"]["senderPublicKey"].as_str().unwrap()
    );
    assert_eq!(
        transaction.fee,
        fixture["data"]["fee"].as_u64().unwrap() as u64
    );
    assert_eq!(
        transaction.amount,
        fixture["data"]["amount"].as_u64().unwrap() as u64
    );
    assert_eq!(
        transaction.recipient_id,
        fixture["data"]["recipientId"].as_str().unwrap()
    );
    assert_eq!(
        transaction.signature,
        fixture["data"]["signature"].as_str().unwrap()
    );
    assert_eq!(transaction.id, fixture["data"]["id"].as_str().unwrap());
}

#[test]
fn test_signed_with_a_second_passphrase() {
    let fixture = json_transaction("transfer", "second-passphrase");
    let serialized = &fixture["serialized"].as_str().unwrap();
    let transaction = deserializer::deserialize(serialized);

    network::set(Network::Devnet);

    assert_eq!(transaction.version, 1);
    assert_eq!(transaction.network, 30);
    assert_eq!(transaction.type_id, Types::Transfer);
    assert_eq!(
        transaction.timestamp,
        fixture["data"]["timestamp"].as_u64().unwrap() as u32
    );
    assert_eq!(
        transaction.sender_public_key,
        fixture["data"]["senderPublicKey"].as_str().unwrap()
    );
    assert_eq!(
        transaction.fee,
        fixture["data"]["fee"].as_u64().unwrap() as u64
    );
    assert_eq!(
        transaction.amount,
        fixture["data"]["amount"].as_u64().unwrap() as u64
    );
    assert_eq!(
        transaction.recipient_id,
        fixture["data"]["recipientId"].as_str().unwrap()
    );
    assert_eq!(
        transaction.signature,
        fixture["data"]["signature"].as_str().unwrap()
    );

    assert_eq!(
        transaction.sign_signature,
        fixture["data"]["signSignature"].as_str().unwrap()
    );
    assert_eq!(transaction.id, fixture["data"]["id"].as_str().unwrap());
}

#[test]
fn test_signed_with_a_passphrase_and_vendor_field() {
    let fixture = json_transaction("transfer", "passphrase-with-vendor-field");
    let serialized = &fixture["serialized"].as_str().unwrap();
    let transaction = deserializer::deserialize(serialized);

    network::set(Network::Devnet);

    assert_eq!(transaction.version, 1);
    assert_eq!(transaction.network, 30);
    assert_eq!(transaction.type_id, Types::Transfer);
    assert_eq!(
        transaction.timestamp,
        fixture["data"]["timestamp"].as_u64().unwrap() as u32
    );
    assert_eq!(
        transaction.sender_public_key,
        fixture["data"]["senderPublicKey"].as_str().unwrap()
    );
    assert_eq!(
        transaction.fee,
        fixture["data"]["fee"].as_u64().unwrap() as u64
    );
    assert_eq!(
        transaction.amount,
        fixture["data"]["amount"].as_u64().unwrap() as u64
    );
    assert_eq!(
        transaction.recipient_id,
        fixture["data"]["recipientId"].as_str().unwrap()
    );
    assert_eq!(
        transaction.signature,
        fixture["data"]["signature"].as_str().unwrap()
    );
    assert_eq!(
        transaction.vendor_field,
        fixture["data"]["vendorField"].as_str().unwrap()
    );
    assert_eq!(transaction.id, fixture["data"]["id"].as_str().unwrap());
}

#[test]
fn test_signed_with_a_second_passphrase_and_vendor_field() {
    let fixture = json_transaction("transfer", "second-passphrase-with-vendor-field");
    let serialized = &fixture["serialized"].as_str().unwrap();
    let transaction = deserializer::deserialize(serialized);

    network::set(Network::Devnet);

    assert_eq!(transaction.version, 1);
    assert_eq!(transaction.network, 30);
    assert_eq!(transaction.type_id, Types::Transfer);
    assert_eq!(
        transaction.timestamp,
        fixture["data"]["timestamp"].as_u64().unwrap() as u32
    );
    assert_eq!(
        transaction.sender_public_key,
        fixture["data"]["senderPublicKey"].as_str().unwrap()
    );
    assert_eq!(
        transaction.fee,
        fixture["data"]["fee"].as_u64().unwrap() as u64
    );
    assert_eq!(
        transaction.amount,
        fixture["data"]["amount"].as_u64().unwrap() as u64
    );
    assert_eq!(
        transaction.recipient_id,
        fixture["data"]["recipientId"].as_str().unwrap()
    );
    assert_eq!(
        transaction.signature,
        fixture["data"]["signature"].as_str().unwrap()
    );
    assert_eq!(
        transaction.sign_signature,
        fixture["data"]["signSignature"].as_str().unwrap()
    );
    assert_eq!(
        transaction.vendor_field,
        fixture["data"]["vendorField"].as_str().unwrap()
    );
    assert_eq!(transaction.id, fixture["data"]["id"].as_str().unwrap());
}

#[test]
fn test_signed_with_a_passphrase_and_vendor_hex_field() {
    let fixture = json_transaction("transfer", "passphrase-with-vendor-field-hex");
    let serialized = &fixture["serialized"].as_str().unwrap();
    let transaction = deserializer::deserialize(serialized);

    network::set(Network::Devnet);

    assert_eq!(transaction.version, 1);
    assert_eq!(transaction.network, 30);
    assert_eq!(transaction.type_id, Types::Transfer);
    assert_eq!(
        transaction.timestamp,
        fixture["data"]["timestamp"].as_u64().unwrap() as u32
    );
    assert_eq!(
        transaction.sender_public_key,
        fixture["data"]["senderPublicKey"].as_str().unwrap()
    );
    assert_eq!(
        transaction.fee,
        fixture["data"]["fee"].as_u64().unwrap() as u64
    );
    assert_eq!(
        transaction.amount,
        fixture["data"]["amount"].as_u64().unwrap() as u64
    );
    assert_eq!(
        transaction.recipient_id,
        fixture["data"]["recipientId"].as_str().unwrap()
    );
    assert_eq!(
        transaction.signature,
        fixture["data"]["signature"].as_str().unwrap()
    );
    assert_eq!(
        transaction.vendor_field_hex,
        fixture["data"]["vendorFieldHex"].as_str().unwrap()
    );
    assert_eq!(transaction.id, fixture["data"]["id"].as_str().unwrap());
}

#[test]
fn test_signed_with_a_second_passphrase_and_vendor_hex_field() {
    let fixture = json_transaction("transfer", "second-passphrase-with-vendor-field-hex");
    let serialized = &fixture["serialized"].as_str().unwrap();
    let transaction = deserializer::deserialize(serialized);

    network::set(Network::Devnet);

    assert_eq!(transaction.version, 1);
    assert_eq!(transaction.network, 30);
    assert_eq!(transaction.type_id, Types::Transfer);
    assert_eq!(
        transaction.timestamp,
        fixture["data"]["timestamp"].as_u64().unwrap() as u32
    );
    assert_eq!(
        transaction.sender_public_key,
        fixture["data"]["senderPublicKey"].as_str().unwrap()
    );
    assert_eq!(
        transaction.fee,
        fixture["data"]["fee"].as_u64().unwrap() as u64
    );
    assert_eq!(
        transaction.amount,
        fixture["data"]["amount"].as_u64().unwrap() as u64
    );
    assert_eq!(
        transaction.recipient_id,
        fixture["data"]["recipientId"].as_str().unwrap()
    );
    assert_eq!(
        transaction.signature,
        fixture["data"]["signature"].as_str().unwrap()
    );
    assert_eq!(
        transaction.sign_signature,
        fixture["data"]["signSignature"].as_str().unwrap()
    );
    assert_eq!(
        transaction.vendor_field_hex,
        fixture["data"]["vendorFieldHex"].as_str().unwrap()
    );
    assert_eq!(transaction.id, fixture["data"]["id"].as_str().unwrap());
}

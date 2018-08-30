use arkecosystem_crypto::configuration::network;
use arkecosystem_crypto::networks::Network;
use arkecosystem_crypto::transactions::{deserializer, serializer};
use *;

#[test]
fn test_signed_with_a_passphrase() {
    network::set(Network::Devnet);

    let fixture = json_transaction("transfer", "passphrase");
    let serialized = fixture["serialized"].as_str().unwrap();
    let transaction = deserializer::deserialize(&serialized);

    let actual = serializer::serialize(&transaction);
    assert_eq!(actual, serialized);
}

#[test]
fn test_signed_with_a_second_passphrase() {
    network::set(Network::Devnet);

    let fixture = json_transaction("transfer", "second-passphrase");
    let serialized = fixture["serialized"].as_str().unwrap();
    let transaction = deserializer::deserialize(&serialized);

    let actual = serializer::serialize(&transaction);
    assert_eq!(actual, serialized);
}

#[test]
fn test_signed_with_a_passphrase_and_vendor_field() {
    network::set(Network::Devnet);

    let fixture = json_transaction("transfer", "passphrase-with-vendor-field");
    let serialized = fixture["serialized"].as_str().unwrap();
    let transaction = deserializer::deserialize(&serialized);

    let actual = serializer::serialize(&transaction);
    assert_eq!(actual, serialized);
}

#[test]
fn test_signed_with_a_second_passphrase_and_vendor_field() {
    network::set(Network::Devnet);

    let fixture = json_transaction("transfer", "second-passphrase-with-vendor-field");
    let serialized = fixture["serialized"].as_str().unwrap();
    let transaction = deserializer::deserialize(&serialized);

    let actual = serializer::serialize(&transaction);
    assert_eq!(actual, serialized);
}

#[test]
fn test_signed_with_a_passphrase_and_vendor_field_hex() {
    network::set(Network::Devnet);

    let fixture = json_transaction("transfer", "passphrase-with-vendor-field-hex");
    let serialized = fixture["serialized"].as_str().unwrap();
    let transaction = deserializer::deserialize(&serialized);

    let actual = serializer::serialize(&transaction);
    assert_eq!(actual, serialized);
}

#[test]
fn test_signed_with_a_second_passphrase_and_vendor_field_hex() {
    network::set(Network::Devnet);

    let fixture = json_transaction("transfer", "second-passphrase-with-vendor-field-hex");
    let serialized = fixture["serialized"].as_str().unwrap();
    let transaction = deserializer::deserialize(&serialized);

    let actual = serializer::serialize(&transaction);
    assert_eq!(actual, serialized);
}

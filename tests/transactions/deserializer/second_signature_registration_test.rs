use arkecosystem_crypto::configuration::network;
use arkecosystem_crypto::enums::types::Types;
use arkecosystem_crypto::enums::Network;
use arkecosystem_crypto::transactions::deserializer;
use *;

#[test]
fn test_signed_with_a_second_passphrase() {
    let fixture = json_transaction("second_signature_registration", "second-passphrase");
    let serialized = &fixture["serialized"].as_str().unwrap();
    let transaction = deserializer::deserialize(serialized);

    network::set(Network::Devnet);

    assert_eq!(transaction.version, 1);
    assert_eq!(transaction.network, 30);
    assert_eq!(transaction.type_id, Types::SecondSignatureRegistration);
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
    // TODO
    // assert_eq!(
    //     transaction.recipient_id,
    //     fixture["data"]["recipientId"].as_str().unwrap()
    // );
    assert_eq!(
        transaction.signature,
        fixture["data"]["signature"].as_str().unwrap()
    );
    assert_eq!(transaction.id, fixture["data"]["id"].as_str().unwrap());

    let asset = fixture["data"]["asset"].clone();
    assert_eq!(transaction.asset, serde_json::from_value(asset).unwrap());
}

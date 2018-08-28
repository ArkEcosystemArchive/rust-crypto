use ::*;
use arkecosystem_crypto::transactions::Transaction;

#[test]
fn test_deserialize() {
    let fixture = json_transaction("transfer", "passphrase");
    let serialized = &fixture["serialized"];
    println!("{:?}", serialized);


}

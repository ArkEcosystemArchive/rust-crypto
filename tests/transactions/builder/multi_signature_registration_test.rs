use arkecosystem_crypto::identities::public_key;
use arkecosystem_crypto::transactions::builder;

#[test]
fn test_signed_with_a_passphrase() {
    let transaction = builder::build_multi_signature_registration(
        "this is a top secret passphrase",
        None,
        2,
        24,
        vec![
            String::from("+03543c6cc3545be6bac09c82721973a052c690658283472e88f24d14739f75acc8"),
            String::from("+0276dc5b8706a85ca9fdc46e571ac84e52fbb48e13ec7a165a80731b44ae89f1fc"),
            String::from("+02e8d5d17eb17bbc8d7bf1001d29a2d25d1249b7bb7a5b7ad8b7422063091f4b31"),
        ],
    );

    assert!(transaction.is_ok());
    assert!(transaction.unwrap().verify());
}

#[test]
fn test_signed_with_a_second_passphrase() {
    let transaction = builder::build_multi_signature_registration(
        "this is a top secret passphrase",
        Some("this is a top secret second passphrase"),
        2,
        24,
        vec![
            String::from("+03543c6cc3545be6bac09c82721973a052c690658283472e88f24d14739f75acc8"),
            String::from("+0276dc5b8706a85ca9fdc46e571ac84e52fbb48e13ec7a165a80731b44ae89f1fc"),
            String::from("+02e8d5d17eb17bbc8d7bf1001d29a2d25d1249b7bb7a5b7ad8b7422063091f4b31"),
        ],
    );

    let second_public_key = public_key::from_passphrase("this is a top secret second passphrase")
        .unwrap()
        .to_string();

    assert!(transaction.is_ok());
    assert!(transaction.as_ref().unwrap().verify());
    assert!(transaction
        .as_ref()
        .unwrap()
        .second_verify(&second_public_key));
}

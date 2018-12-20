use arkecosystem_crypto::identities::public_key;
use arkecosystem_crypto::transactions::builder;

#[test]
fn test_signed_with_a_passphrase() {
    let transaction =
        builder::build_delegate_registration("this is a top secret passphrase", None, "dummy");

    assert!(transaction.is_ok());
    assert!(transaction.unwrap().verify());
}

#[test]
fn test_signed_with_a_second_passphrase() {
    let transaction = builder::build_delegate_registration(
        "this is a top secret passphrase",
        Some("this is a top secret second passphrase"),
        "dummy",
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

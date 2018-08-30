use arkecosystem_crypto::transactions::builder;
use *;

#[test]
fn test_signed_with_a_passphrase() {
    let transaction = builder::build_vote(
        "this is a top secret passphrase",
        None,
        vec![String::from(
            "+034151a3ec46b5670a682b0a63394f863587d1bc97483b1b6c70eb58e7f0aed192",
        )],
    );

    assert!(transaction.is_ok());
    assert!(transaction.unwrap().verify());
}

#[test]
fn test_signed_with_a_second_passphrase() {
    let transaction = builder::build_vote(
        "this is a top secret passphrase",
        Some("this is a top secret passphrase"),
        vec![String::from(
            "+034151a3ec46b5670a682b0a63394f863587d1bc97483b1b6c70eb58e7f0aed192",
        )],
    );

    assert!(transaction.is_ok());
    assert!(transaction.as_ref().unwrap().verify());
    assert!(transaction.as_ref().unwrap().second_verify());
}

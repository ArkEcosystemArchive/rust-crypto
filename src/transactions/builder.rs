use failure;
use hex;

use configuration::fees;
use enums::TransactionType;
use identities::{address, public_key};
use transactions::transaction::{Asset, Transaction};
use utils::slot;

pub fn build_transfer(
    passphrase: &str,
    second_passphrase: Option<&str>,
    recipient_id: &str,
    amount: u64,
    vendor_field: &str,
) -> Result<Transaction, failure::Error> {
    let mut transaction = create(TransactionType::Transfer);
    transaction.recipient_id = recipient_id.to_owned();
    transaction.amount = amount;
    transaction.vendor_field = vendor_field.to_owned();

    Ok(sign(transaction, passphrase, second_passphrase))
}

pub fn build_second_signature_registration(
    passphrase: &str,
    second_passphrase: &str,
) -> Result<Transaction, failure::Error> {
    let mut transaction = create(TransactionType::SecondSignatureRegistration);

    transaction.asset = Asset::Signature {
        public_key: hex::encode(
            public_key::from_passphrase(second_passphrase)?
                .serialize()
                .to_vec(),
        ),
    };

    Ok(sign(transaction, passphrase, Some(second_passphrase)))
}

pub fn build_delegate_registration(
    passphrase: &str,
    second_passphrase: Option<&str>,
    username: &str,
) -> Result<Transaction, failure::Error> {
    let mut transaction = create(TransactionType::DelegateRegistration);

    transaction.asset = Asset::Delegate {
        username: username.to_owned(),
    };

    Ok(sign(transaction, passphrase, second_passphrase))
}

pub fn build_vote(
    passphrase: &str,
    second_passphrase: Option<&str>,
    votes: Vec<String>,
) -> Result<Transaction, failure::Error> {
    let mut transaction = create(TransactionType::Vote);
    transaction.asset = Asset::Votes(votes);
    transaction.recipient_id = address::from_passphrase(passphrase, None)?;

    Ok(sign(transaction, passphrase, second_passphrase))
}

pub fn build_multi_signature_registration(
    passphrase: &str,
    second_passphrase: Option<&str>,
    min: u8,
    lifetime: u8,
    keysgroup: Vec<String>,
) -> Result<Transaction, failure::Error> {
    let mut transaction = create(TransactionType::MultiSignatureRegistration);

    let len = (keysgroup.len() + 1) as u64;
    transaction.fee = len * fees::get(TransactionType::MultiSignatureRegistration);
    transaction.asset = Asset::MultiSignatureRegistration {
        min,
        lifetime,
        keysgroup,
    };

    Ok(sign(transaction, passphrase, second_passphrase))
}

fn sign(
    mut transaction: Transaction,
    passphrase: &str,
    second_passphrase: Option<&str>,
) -> Transaction {
    transaction.timestamp = slot::get_time();
    transaction.sign(passphrase);

    if second_passphrase.is_some() {
        transaction.second_sign(second_passphrase.unwrap());
    }

    transaction.id = transaction.get_id();
    transaction
}

fn create(transaction_type: TransactionType) -> Transaction {
    let mut transaction = Transaction::default();
    transaction.type_id = transaction_type;
    transaction.fee = fees::get(transaction_type);
    transaction
}

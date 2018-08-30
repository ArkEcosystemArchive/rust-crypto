use failure;

use configuration::fees;
use enums::TransactionType;
use identities::address;
use transactions::transaction::{Asset, Transaction};
use utils::slot;

pub fn build_vote(
    passphrase: &str,
    second_passphrase: Option<&str>,
    votes: Vec<String>,
) -> Result<Transaction, failure::Error> {
    let mut transaction = create(TransactionType::Vote);
    transaction.asset = Asset::Votes(votes);
    transaction.recipient_id = address::from_passphrase(passphrase)?;

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

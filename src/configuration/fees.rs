use enums::TransactionType;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    static ref FEES: Mutex<HashMap<TransactionType, u64>> = {
        let mut m = HashMap::new();
        m.insert(TransactionType::Transfer, TransactionType::Transfer.fee());
        m.insert(TransactionType::SecondSignatureRegistration, TransactionType::SecondSignatureRegistration.fee());
        m.insert(TransactionType::DelegateRegistration, TransactionType::DelegateRegistration.fee());
        m.insert(TransactionType::Vote, TransactionType::Vote.fee());
        m.insert(TransactionType::MultiSignatureRegistration, TransactionType::MultiSignatureRegistration.fee());
        m.insert(TransactionType::Ipfs, TransactionType::Ipfs.fee());
        m.insert(TransactionType::TimelockTransfer, TransactionType::TimelockTransfer.fee());
        m.insert(TransactionType::MultiPayment, TransactionType::MultiPayment.fee());
        m.insert(TransactionType::DelegateResignation, TransactionType::DelegateResignation.fee());

        Mutex::new(m)
    };
}

pub fn get(transaction_type: TransactionType) -> u64 {
    FEES.lock()
        .unwrap()
        .get(&transaction_type)
        .cloned()
        .unwrap()
}

pub fn set(transaction_type: TransactionType, value: u64) {
    FEES.lock().unwrap().insert(transaction_type, value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_fee() {
        assert_eq!(get(TransactionType::Vote), TransactionType::Vote.fee());
    }

    #[test]
    #[ignore]
    fn set_fee() {
        set(TransactionType::Vote, 0);
        assert_eq!(get(TransactionType::Vote), 0);
    }
}

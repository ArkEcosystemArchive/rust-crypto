use enums::types::Types;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    static ref FEES: Mutex<HashMap<Types, u64>> = {
        let mut m = HashMap::new();
        m.insert(Types::Transfer, Types::Transfer.fee());
        m.insert(Types::SecondSignatureRegistration, Types::SecondSignatureRegistration.fee());
        m.insert(Types::DelegateRegistration, Types::DelegateRegistration.fee());
        m.insert(Types::Vote, Types::Vote.fee());
        m.insert(Types::MultiSignatureRegistration, Types::MultiSignatureRegistration.fee());
        m.insert(Types::Ipfs, Types::Ipfs.fee());
        m.insert(Types::TimelockTransfer, Types::TimelockTransfer.fee());
        m.insert(Types::MultiPayment, Types::MultiPayment.fee());
        m.insert(Types::DelegateResignation, Types::DelegateResignation.fee());

        Mutex::new(m)
    };
}

pub fn get(transaction_type: Types) -> u64 {
    FEES.lock()
        .unwrap()
        .get(&transaction_type)
        .cloned()
        .unwrap()
}

pub fn set(transaction_type: Types, value: u64) {
    FEES.lock().unwrap().insert(transaction_type, value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_fee() {
        assert_eq!(get(Types::Vote), Types::Vote.fee());
    }

    #[test]
    #[ignore]
    fn set_fee() {
        set(Types::Vote, 0);
        assert_eq!(get(Types::Vote), 0);
    }

}

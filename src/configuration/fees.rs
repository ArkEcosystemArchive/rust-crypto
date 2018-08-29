use enums::fees;
use enums::types::Types;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref FEES: Mutex<HashMap<Types, u32>> = {
        let mut m = HashMap::new();
        m.insert(Types::Transfer, fees::TRANSFER);
        m.insert(
            Types::SecondSignatureRegistration,
            fees::SECOND_SIGNATURE_REGISTRATION,
        );
        m.insert(Types::DelegateRegistration, fees::DELEGATE_REGISTRATION);
        m.insert(Types::Vote, fees::VOTE);
        m.insert(
            Types::MultiSignatureRegistration,
            fees::MULTI_SIGNATURE_REGISTRATION,
        );
        m.insert(Types::Ipfs, fees::IPFS);
        m.insert(Types::TimelockTransfer, fees::TIMELOCK_TRANSFER);
        m.insert(Types::MultiPayment, fees::MULTI_PAYMENT);
        m.insert(Types::DelegateResignation, fees::DELEGATE_RESIGNATION);

        Mutex::new(m)
    };
}

pub fn get(transaction_type: Types) -> u32 {
    FEES.lock()
        .unwrap()
        .get(&transaction_type)
        .cloned()
        .unwrap()
}

pub fn set(transaction_type: Types, value: u32) {
    FEES.lock().unwrap().insert(transaction_type, value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_fee() {
        assert_eq!(get(Types::Vote), fees::VOTE);
    }

    #[test]
    fn set_fee() {
        set(Types::Vote, 0);
        assert_eq!(get(Types::Vote), 0);
    }

}

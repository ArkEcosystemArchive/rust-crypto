use std::mem::transmute;

enum_number!(TransactionType {
    Transfer = 0,
    SecondSignatureRegistration = 1,
    DelegateRegistration = 2,
    Vote = 3,
    MultiSignatureRegistration = 4,
    Ipfs = 5,
    TimelockTransfer = 6,
    MultiPayment = 7,
    DelegateResignation = 8,
});

impl TransactionType {
    pub fn fee(self) -> u64 {
        match self {
            TransactionType::Transfer => 10_000_000,
            TransactionType::SecondSignatureRegistration => 500_000_000,
            TransactionType::DelegateRegistration => 2_500_000_000,
            TransactionType::Vote => 100_000_000,
            TransactionType::MultiSignatureRegistration => 500_000_000,
            TransactionType::Ipfs => 0,
            TransactionType::TimelockTransfer => 0,
            TransactionType::MultiPayment => 0,
            TransactionType::DelegateResignation => 0,
        }
    }
}

impl Default for TransactionType {
    fn default() -> TransactionType {
        TransactionType::Transfer
    }
}

impl From<u8> for TransactionType {
    fn from(t: u8) -> TransactionType {
        assert!(
            TransactionType::Transfer as u8 <= t && t <= TransactionType::DelegateResignation as u8
        );
        unsafe { transmute(t) }
    }
}

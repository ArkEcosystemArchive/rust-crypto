use std::mem::transmute;

enum_number!(Types {
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

impl Types {
    pub fn fee(&self) -> u64 {
        match self {
            Types::Transfer => 10_000_000,
            Types::SecondSignatureRegistration => 500_000_000,
            Types::DelegateRegistration => 2_500_000_000,
            Types::Vote => 100_000_000,
            Types::MultiSignatureRegistration => 500_000_000,
            Types::Ipfs => 0,
            Types::TimelockTransfer => 0,
            Types::MultiPayment => 0,
            Types::DelegateResignation => 0,
        }
    }
}

impl Default for Types {
    fn default() -> Types {
        Types::Transfer
    }
}

impl From<u8> for Types {
    fn from(t: u8) -> Types {
        assert!(Types::Transfer as u8 <= t && t <= Types::DelegateResignation as u8);
        unsafe { transmute(t) }
    }
}

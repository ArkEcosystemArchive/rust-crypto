use std::mem::transmute;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Types {
    Transfer = 0,
    SecondSignatureRegistration = 1,
    DelegateRegistration = 2,
    Vote = 3,
    MultiSignatureRegistration = 4,
    Ipfs = 5,
    TimelockTransfer = 6,
    MultiPayment = 7,
    DelegateResignation = 8,
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

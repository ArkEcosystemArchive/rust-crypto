use chrono::prelude::*;
use configuration::network;

pub fn get_time() -> u32 {
    let epoch = DateTime::parse_from_rfc3339(network::get().epoch())
        .unwrap()
        .timestamp();
    let now = Utc::now().timestamp();

    (now - epoch) as u32
}

pub fn get_epoch() -> u32 {
    DateTime::parse_from_rfc3339(network::get().epoch())
        .unwrap()
        .timestamp() as u32
}

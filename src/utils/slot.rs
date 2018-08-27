use chrono::prelude::*;
use configuration::network;

pub fn get_time() -> i64 {
    let epoch = DateTime::parse_from_rfc3339(network::get().epoch()).unwrap().timestamp();
    let now = Utc::now().timestamp();

    now - epoch
}

pub fn get_epoch() -> i64 {
    DateTime::parse_from_rfc3339(network::get().epoch()).unwrap().timestamp()
}

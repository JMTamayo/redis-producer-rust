use std::collections::BTreeMap;

use super::super::models::models::Timestamp;
use chrono;
use structmap::ToMap;

pub fn map_message (message: Timestamp) -> BTreeMap<String, String> {

    let map = Timestamp::to_stringmap(message);

    return map;

}

pub fn get_message() -> Timestamp {

    let datetime_local = chrono::Local::now();
    let datetime_utc = datetime_local.naive_utc();

    return Timestamp {
        datetime_local: datetime_local.to_string(),
        datetime_utc: datetime_utc.to_string(),
    };

}
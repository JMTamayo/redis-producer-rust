use structmap::ToMap;
use structmap_derive::ToMap;

#[derive(ToMap, Default)]
pub struct Timestamp {
    pub datetime_local: String,
    pub datetime_utc: String,
}
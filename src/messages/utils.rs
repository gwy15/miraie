use crate::{DateTime, Result};
use chrono::{FixedOffset, NaiveDateTime};
use serde::Deserializer;
use serde_json::Value;

/// parse timestamp to datetime
pub fn _parse_dt<'de, D>(deserializer: D) -> ::std::result::Result<DateTime, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::{self, Deserialize, Deserializer, Serializer};
    let timestamp = i64::deserialize(deserializer)?;

    let dt = DateTime::from_utc(
        NaiveDateTime::from_timestamp(timestamp, 0),
        FixedOffset::east(8 * 3600),
    );

    Ok(dt)
}

pub fn remove_string(v: &mut Value, name: &str) -> Option<String> {
    match v.as_object_mut()?.remove(name) {
        Some(Value::String(s)) => Some(s),
        _ => None,
    }
}

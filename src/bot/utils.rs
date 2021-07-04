use serde::{Deserialize, Deserializer};

pub fn from_string_ignore_error<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Number(T),
    }

    match StringOrInt::<i64>::deserialize(deserializer)? {
        StringOrInt::String(s) => {
            if s.is_empty() {
                return Ok(None);
            }
            s.parse::<i64>().map(Some).map_err(serde::de::Error::custom)
        }
        StringOrInt::Number(i) => Ok(Some(i)),
    }
}

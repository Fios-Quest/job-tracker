use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use std::ops::Deref;

const FORMAT: &str = "%Y-%m-%dT%H:%M";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timestamp(#[serde(with = "timestamp_serde")] DateTime<Utc>);

impl Timestamp {
    pub fn from_timestamp(time: i64) -> Self {
        Timestamp(DateTime::from_timestamp(time, 0).expect("Given timestamp is out of range"))
    }

    pub fn from_string<S: AsRef<str>>(time: S) -> Self {
        let dt = NaiveDateTime::parse_from_str(time.as_ref(), FORMAT).expect("Invalid format");
        Timestamp(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }

    #[cfg(any(test, feature = "desktop"))]
    pub fn now() -> Self {
        Timestamp::from_timestamp(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards!")
                .as_secs() as i64,
        )
    }
}

impl Deref for Timestamp {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

mod timestamp_serde {
    use super::*;
    use chrono::NaiveDateTime;
    use serde::Serializer;

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}

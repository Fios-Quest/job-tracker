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
        // ToDo: We should accept a series of possible formats here
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

    pub fn looks_valid(self) -> bool {
        self.0.timestamp() > 0
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

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum TempDateTimeInfo {
        TimeStamp(i64),
        DateTimeString(String),
    }

    impl TempDateTimeInfo {
        fn to_datetime(&self) -> anyhow::Result<DateTime<Utc>> {
            match self {
                Self::TimeStamp(i) => DateTime::<Utc>::from_timestamp(*i, 0)
                    .ok_or(anyhow::anyhow!("Invalid timestamp")),
                Self::DateTimeString(s) => {
                    let dt = NaiveDateTime::parse_from_str(s, FORMAT)?;
                    Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                }
            }
        }
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
        TempDateTimeInfo::deserialize(deserializer)?
            .to_datetime()
            .map_err(|e| serde::de::Error::custom(format!("{e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_timestamp() {
        let timestamp = Timestamp::from_timestamp(-14182980);
        let as_string = format!("{}", timestamp.format("%Y-%m-%d %H:%M:%S"));
        assert_eq!(as_string, "1969-07-20 20:17:00");
    }

    #[test]
    fn test_from_string() {
        let timestamp = Timestamp::from_string("1969-07-20T20:17");
        assert_eq!(timestamp.timestamp(), -14182980);
    }

    #[test]
    fn test_now() {
        let timestamp = Timestamp::now();
        assert!(timestamp.timestamp() > 1754300623); // Time when test was written
    }

    #[test]
    fn test_looks_valid() {
        assert!(!Timestamp::from_string("1969-07-20T20:17").looks_valid());
        assert!(Timestamp::now().looks_valid());
    }

    #[test]
    fn test_serialize() {
        let timestamp = Timestamp::from_timestamp(-14182980);
        let serialized = serde_json::to_string(&timestamp).unwrap();
        assert_eq!(serialized, "\"1969-07-20T20:17\"");
    }

    #[test]
    fn test_deserialize() {
        let string = "\"1969-07-20T20:17\"";
        let timestamp_from_string: Timestamp = serde_json::from_str(string).unwrap();
        assert_eq!(timestamp_from_string.timestamp(), -14182980);

        let number = "-14182980";
        let timestamp_from_number: Timestamp = serde_json::from_str(number).unwrap();
        let as_string = format!("{}", timestamp_from_number.format("%Y-%m-%d %H:%M:%S"));
        assert_eq!(as_string, "1969-07-20 20:17:00");
    }
}

use core::ops::Deref;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timestamp(i64);

impl Timestamp {
    pub fn new(time: i64) -> Self {
        Timestamp(time)
    }

    #[cfg(any(test, feature = "desktop"))]
    pub fn now() -> Self {
        Timestamp::new(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards!")
                .as_secs() as i64,
        )
    }
}

impl From<i64> for Timestamp {
    fn from(value: i64) -> Self {
        Timestamp(value)
    }
}

impl From<Timestamp> for i64 {
    fn from(value: Timestamp) -> Self {
        value.0
    }
}

impl Deref for Timestamp {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use core::ops::Deref;
#[cfg(feature = "use_std")]
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Timestamp(i64);

impl Timestamp {
    pub fn new(time: i64) -> Self {
        Timestamp(time)
    }

    #[cfg(feature = "use_std")]
    pub fn now() -> Self {
        Timestamp::new(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
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

impl Deref for Timestamp {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

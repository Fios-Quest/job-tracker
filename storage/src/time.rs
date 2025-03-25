use core::ops::Deref;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
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

impl Deref for Timestamp {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

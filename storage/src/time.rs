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

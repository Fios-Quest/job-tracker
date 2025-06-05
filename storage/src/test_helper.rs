use anyhow::Result;
use std::sync::Mutex;

pub trait TestHelper: Sized {
    async fn new_test() -> Result<Self>;
}

pub struct TestCounter(Mutex<usize>);

impl TestCounter {
    pub const fn new() -> Self {
        Self(Mutex::new(0))
    }

    pub fn next(&self) -> usize {
        *self.0.lock().expect("Mutex lock dropped by another thread") + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    mod test_counter {
        use crate::test_helper::TestCounter;

        const NEW_COUNTER: TestCounter = TestCounter::new();

        fn test_new_counter() {
            assert_eq!(NEW_COUNTER.next(), 1);
        }

        fn test_next_counter() {
            let counter = TestCounter::new();
            assert_eq!(NEW_COUNTER.next(), 1);
            assert_eq!(NEW_COUNTER.next(), 2);
            assert_eq!(NEW_COUNTER.next(), 3);
        }
    }
}

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
        let mut next = self.0.lock().expect("Mutex lock dropped by another thread");
        *next += 1;
        *next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    mod test_counter {
        use crate::test_helper::TestCounter;

        const NEW_COUNTER: TestCounter = TestCounter::new();

        #[test]
        fn test_new_counter() {
            assert_eq!(NEW_COUNTER.next(), 1);
        }

        #[test]
        fn test_non_const() {
            let counter = TestCounter::new();
            assert_eq!(counter.next(), 1);
        }

        // #[test]
        // fn test_next_counter() {
        //     let counter = TestCounter::new();
        //     assert_eq!(NEW_COUNTER.next(), 1);
        //     assert_eq!(NEW_COUNTER.next(), 2);
        //     assert_eq!(NEW_COUNTER.next(), 3);
        // }
    }
}

use anyhow::Result;
use std::sync::Mutex;

pub trait TestHelper: Sized {
    async fn new_test() -> Result<Self>;
}

macro_rules! test_base_store {
    ($storage:ident, $storable:ident) => {
        paste! {
            #[tokio::test]
            async fn [< test_base_store_ $storage:snake _with_ $storable:snake >] () {
                let mut test_subject = $storage::new_test().await.expect("Could not create storage");
                let storable = $storable::new_test().await.expect("Could not create storable");
                test_subject.store(storable.clone()).await.expect("Could not store storable in storage");
                let recalled_storable = test_subject.recall_by_id(&storable.get_id()).await.expect("Could not recall storable from storage by id");
                assert_eq!(storable, recalled_storable);
            }
        }
    };
}

pub(crate) use test_base_store;

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

        const new_counter: TestCounter = TestCounter::new();

        fn test_new_counter() {
            assert_eq!(new_counter.next(), 1);
        }

        fn test_next_counter() {
            let counter = TestCounter::new();
            assert_eq!(new_counter.next(), 1);
            assert_eq!(new_counter.next(), 2);
            assert_eq!(new_counter.next(), 3);
        }
    }
}

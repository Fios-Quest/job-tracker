use crate::composite_store::ThreadSafeGeneralStore;
use crate::prelude::{Company, Flag, Interview, Question, Role, Value};
use crate::storage::StubStore;

pub type StubThreadSafeGeneralStore = ThreadSafeGeneralStore<
    StubStore<Company>,
    StubStore<Flag>,
    StubStore<Value>,
    StubStore<Role>,
    StubStore<Interview>,
    StubStore<Question>,
>;

impl StubThreadSafeGeneralStore {
    pub fn new_stub() -> Self {
        Self::default()
    }
}

impl Default for StubThreadSafeGeneralStore {
    fn default() -> Self {
        StubThreadSafeGeneralStore::new(
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
        )
    }
}

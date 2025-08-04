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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use crate::test_helper::TestHelper;

    #[tokio::test]
    async fn test_new_stub() {
        let mut stub_store = StubThreadSafeGeneralStore::new_stub();
        let company = Company::new_test().await.unwrap();
        stub_store.store(company.clone()).await.unwrap();
        let recalled_company: Company = stub_store.recall_by_id(company.id).await.unwrap();
        assert_eq!(company, recalled_company);
    }
}

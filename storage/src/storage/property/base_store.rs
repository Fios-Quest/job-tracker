use crate::storable::HasId;
use anyhow::Result;

pub trait BaseStore<T>
where
    T: HasId + Clone,
{
    async fn store(&mut self, storable: T) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storable::{Company, Flag};
    use crate::storage::{RecallById, StubStore};

    #[tokio::test]
    async fn test_company() {
        let company = Company::new("company");
        let mut store = StubStore::default();
        store.store(company.clone()).await.unwrap();
        let recalled_company = store.recall_by_id(&company).await.unwrap();
        assert_eq!(recalled_company, company);
    }

    #[tokio::test]
    async fn test_flag() {
        let company = Company::new("company");
        let flag = Flag::new_green(company.id, "green flag".to_string());
        let mut store = StubStore::default();
        store.store(flag.clone()).await.unwrap();
        let recalled_flag = store.recall_by_id(&flag).await.unwrap();
        assert_eq!(recalled_flag, flag);
    }
}

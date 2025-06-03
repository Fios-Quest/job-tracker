use crate::storable::property::has_id::HasId;
use anyhow::Result;

pub trait BaseStore<T>
where
    T: HasId + Clone,
{
    async fn store(&mut self, storable: T) -> Result<()>;

    async fn recall_by_id<I: HasId>(&self, id: &I) -> Result<T>;
}

#[cfg(test)]
mod tests {
    use crate::storable::object::company::Company;
    use crate::storable::object::flag::Flag;
    use crate::storage::medium::stub_storage::StubStore;
    use crate::storage::property::base_store::BaseStore;
    use crate::storage::property::recall_by_company::RecallByCompany;

    #[tokio::test]
    async fn test_company() {
        let company = Company::new("company");
        let mut store = StubStore::default();
        store.store(company.clone()).await.unwrap();
        let recalled_company = store.recall_by_id(&company);
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

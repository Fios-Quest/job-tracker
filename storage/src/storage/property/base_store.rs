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

#[cfg(test)]
pub mod test_helper {
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
}

use crate::storable::*;
use anyhow::Result;

pub trait RecallById<T>
where
    T: HasId + HasDeleted + Clone,
{
    async fn recall_by_id<I: HasId>(&self, id: I) -> Result<T>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storable::{Company, Flag};
    use crate::storage::{BaseStore, StubStore};

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
    macro_rules! test_recall_by_id {
        ($storage:ident, $storable:ident) => {
            paste! {
                #[tokio::test]
                async fn [< test_recall_by_id_ $storage:snake _with_ $storable:snake >] () {
                    use crate::Timestamp;

                    let mut test_subject = $storage::new_test().await.expect("Could not create storage");
                    let mut storable = $storable::new_test().await.expect("Could not create storable");
                    test_subject.store(storable.clone()).await.expect("Could not store storable in storage");

                    let recalled_storable = test_subject.recall_by_id(&storable.get_id()).await.expect("Could not recall storable from storage by id");
                    assert_eq!(storable, recalled_storable);

                    storable.date_deleted = Some(Timestamp::now());
                    test_subject.store(storable.clone()).await.expect("Could not store storable in storage");

                    let result: Result<$storable, _> = test_subject.recall_by_id(&storable.get_id()).await;
                    assert!(result.is_err());
                }
            }
        };
    }

    pub(crate) use test_recall_by_id;
}

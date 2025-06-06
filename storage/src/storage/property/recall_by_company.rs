use crate::prelude::HasDeleted;
use crate::storable::{HasCompany, HasId};
use anyhow::Result;

pub trait RecallByCompany<T>
where
    T: HasCompany + HasDeleted + Clone,
{
    async fn recall_by_company<I: HasId>(&self, company_id: I) -> Result<Vec<T>>;
}

#[cfg(test)]
pub mod test_helper {
    macro_rules! test_recall_by_company {
        ($storage:ident, $storable:ident) => {
            paste! {
                #[tokio::test]
                async fn [< test_recall_by_company_ $storage:snake _with_ $storable:snake >] () {
                    use crate::Timestamp;

                    let mut test_subject = $storage::new_test().await.expect("Could not create storage");
                    let mut storable = $storable::new_test().await.expect("Could not create storable");
                    test_subject.store(storable.clone()).await.expect("Could not store storable in storage");

                    let recalled_storable = test_subject.recall_by_company(&storable.get_company_id()).await.expect("Could not recall storable from storage by company id");
                    assert_eq!(recalled_storable.len(), 1);
                    assert!(recalled_storable.contains(&storable));

                    storable.date_deleted = Some(Timestamp::now());
                    test_subject.store(storable.clone()).await.expect("Could not store storable in storage");

                    let v: Vec<$storable> = test_subject.recall_by_company(&storable.get_company_id()).await.expect("Could not recall storable from storage by company id");
                    assert!(v.is_empty());
                }
            }
        };
    }

    pub(crate) use test_recall_by_company;
}

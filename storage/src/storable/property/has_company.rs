use uuid::Uuid;

pub trait HasCompany {
    fn get_company_id(&self) -> Uuid;
}

impl<T> HasCompany for &T
where
    T: HasCompany,
{
    fn get_company_id(&self) -> Uuid {
        (*self).get_company_id()
    }
}

#[cfg(test)]
pub mod test_helper {
    macro_rules! test_has_company {
        ($storable:ident) => {
            paste! {
                #[tokio::test]
                async fn [< test_has_company_ $storable:snake >] () {
                    let storable = $storable::new_test().await.expect("Could not create storable");
                    // Not the best test TBH, might break if we stop using v4 uuids
                    assert_eq!(storable.get_company_id().get_version(), Some(uuid::Version::Random));
                }
            }
        };
    }

    pub(crate) use test_has_company;
}

pub trait HasDeleted {
    fn is_deleted(&self) -> bool;
}
impl<T> HasDeleted for &T
where
    T: HasDeleted,
{
    fn is_deleted(&self) -> bool {
        (*self).is_deleted()
    }
}

macro_rules! impl_has_deleted {
    ($storable:ty) => {
        impl HasDeleted for $storable {
            fn is_deleted(&self) -> bool {
                self.date_deleted.is_some()
            }
        }
    };
}
pub(crate) use impl_has_deleted;

#[cfg(test)]
pub mod test_helper {
    macro_rules! test_has_deleted {
        ($storable:ty) => {
            paste! {
                #[tokio::test]
                async fn [< test_has_deleted_ $storable:snake >] () {
                    use crate::Timestamp;
                    let mut storable = $storable::new_test().await.expect("Could not create storable");
                    assert!(!storable.is_deleted());
                    storable.date_deleted = Some(Timestamp::now());
                    assert!(storable.is_deleted());
                }
            }
        };
    }
    pub(crate) use test_has_deleted;
}

#[cfg(test)]
pub(crate) use test_helper::test_has_deleted;

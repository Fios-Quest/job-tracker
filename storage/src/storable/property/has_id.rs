use uuid::Uuid;

pub trait HasId {
    fn get_id(&self) -> Uuid;
}

impl<T> HasId for &T
where
    T: HasId,
{
    fn get_id(&self) -> Uuid {
        (*self).get_id()
    }
}

impl HasId for Uuid {
    fn get_id(&self) -> Uuid {
        *self
    }
}

macro_rules! impl_has_id {
    ($storable:ty) => {
        impl HasId for $storable {
            fn get_id(&self) -> Uuid {
                self.id
            }
        }
    };
}
pub(crate) use impl_has_id;

#[cfg(test)]
pub mod test_helper {

    macro_rules! test_has_id {
        ($storable:ident) => {
            paste! {
                #[tokio::test]
                async fn [< test_has_id_ $storable:snake >] () {
                    let storable = $storable::new_test().await.expect("Could not create storable");
                    // Not the best test TBH, might break if we stop using v4 uuids
                    assert_eq!(storable.get_id().get_version(), Some(uuid::Version::Random));
                }
            }
        };
    }
    pub(crate) use test_has_id;
}

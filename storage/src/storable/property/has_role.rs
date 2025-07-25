use uuid::Uuid;

pub trait HasRole {
    fn get_role_id(&self) -> Uuid;
}

impl<T> HasRole for &T
where
    T: HasRole,
{
    fn get_role_id(&self) -> Uuid {
        (*self).get_role_id()
    }
}

macro_rules! impl_has_role {
    ($storable:ty) => {
        impl HasRole for $storable {
            fn get_role_id(&self) -> Uuid {
                self.role_id
            }
        }
    };
}
pub(crate) use impl_has_role;

#[cfg(test)]
pub mod test_helper {
    macro_rules! test_has_role {
        ($storable:ty) => {
            paste! {
                #[tokio::test]
                async fn [< test_has_role_ $storable:snake >] () {
                    let storable = $storable::new_test().await.expect("Could not create storable");
                    // Not the best test TBH, might break if we stop using v4 uuids
                    assert_eq!(storable.get_role_id().get_version(), Some(uuid::Version::Random));
                }
            }
        };
    }
    pub(crate) use test_has_role;
}

#[cfg(test)]
pub(crate) use test_helper::test_has_role;

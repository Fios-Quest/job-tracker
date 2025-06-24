pub trait HasName {
    fn get_name(&self) -> &str;
}

impl<T> HasName for &T
where
    T: HasName,
{
    fn get_name(&self) -> &str {
        (*self).get_name()
    }
}

#[macro_export]
macro_rules! impl_has_name {
    ($storable:ty) => {
        impl HasName for $storable {
            fn get_name(&self) -> &str {
                &self.name
            }
        }
    };
}

#[cfg(test)]
pub mod test_helper {
    macro_rules! test_has_name {
        ($storable:ident) => {
            paste! {
                #[tokio::test]
                async fn [< test_has_name_ $storable:snake >] () {
                    let storable = $storable::new_test().await.expect("Could not create storable");
                    assert!(!storable.get_name().is_empty());
                }
            }
        };
    }

    pub(crate) use test_has_name;
}

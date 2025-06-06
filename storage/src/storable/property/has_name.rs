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

impl HasName for &str {
    fn get_name(&self) -> &str {
        self
    }
}

impl HasName for String {
    fn get_name(&self) -> &str {
        self
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str() {
        let s = "Hello";
        assert_eq!(s, s.get_name());
    }

    #[test]
    fn test_string() {
        let s = "Hello".to_string();
        assert_eq!(s, s.get_name());
    }
}

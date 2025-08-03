use partially::Partial;

pub trait IsPartialComplete: Partial {
    fn is_complete(&self) -> bool;
}

macro_rules! impl_is_partial_complete_optional_name_only {
    ($storable:ty) => {
        impl IsPartialComplete for $storable {
            fn is_complete(&self) -> bool {
                self.name.as_ref().map(|name| name.is_empty()) == Some(false)
            }
        }
    };
}
pub(crate) use impl_is_partial_complete_optional_name_only;

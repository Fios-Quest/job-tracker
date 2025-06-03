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

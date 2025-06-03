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

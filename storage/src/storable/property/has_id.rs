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

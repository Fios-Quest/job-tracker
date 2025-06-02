use crate::Company;
use uuid::Uuid;

pub trait HasId {
    fn id(&self) -> Uuid;
}

impl HasId for Uuid {
    fn id(&self) -> Uuid {
        *self
    }
}

impl HasId for Company {
    fn id(&self) -> Uuid {
        self.id
    }
}

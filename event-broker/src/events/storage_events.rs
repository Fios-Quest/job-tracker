use storage::prelude::HasId;
use uuid::Uuid;

pub struct Store<T: HasId + Clone> {
    storable: T,
}

impl<T: HasId + Clone> Store<T> {
    pub fn new(storable: T) -> Self {
        Self { storable }
    }
}

impl<T: HasId + Clone> HasId for Store<T> {
    fn get_id(&self) -> Uuid {
        self.storable.get_id()
    }
}

pub struct Updated<T: HasId + Clone> {
    storable: T,
}

impl<T: HasId + Clone> Updated<T> {
    pub fn new(storable: T) -> Self {
        Self { storable }
    }
}

impl<T: HasId + Clone> HasId for Updated<T> {
    fn get_id(&self) -> Uuid {
        self.storable.get_id()
    }
}

#[cfg(test)]
mod tests {
    use crate::events::{Store, Updated};
    use storage::prelude::*;

    #[test]
    fn test_has_id_for_store_event() {
        let company = Company::new("Test Company");
        let event = Store::new(company.clone());
        assert_eq!(event.get_id(), company.get_id())
    }

    #[test]
    fn test_has_id_for_updated_event() {
        let company = Company::new("Test Company");
        let event = Updated::new(company.clone());
        assert_eq!(event.get_id(), company.get_id())
    }
}

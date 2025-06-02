use crate::{HasId, StorageError};
use anyhow::Result;

pub trait BetterStore<T> {
    fn store(&mut self, item: &T) -> Result<()>;
}

pub trait StoreById<I, T> {
    fn get_by_id(&self, id: &I) -> Result<T>;
}

struct StubStore<T> {
    store: Vec<T>,
}

impl<T> StubStore<T>
where
    T: HasId,
{
    fn new() -> StubStore<T> {
        StubStore { store: Vec::new() }
    }
}

impl<T> BetterStore<T> for StubStore<T>
where
    T: HasId + Clone,
{
    fn store(&mut self, item: &T) -> Result<()> {
        // Remove the item if its already stored
        self.store.retain(|store_item| item.id() != store_item.id());

        // Store the new item
        self.store.push(item.clone());

        Ok(())
    }
}

impl<I, T> StoreById<I, T> for StubStore<T>
where
    I: HasId,
    T: HasId + Clone,
{
    fn get_by_id(&self, id: &I) -> Result<T> {
        Ok(self
            .store
            .iter()
            .find(|store_item| store_item.id() == id.id())
            .cloned()
            .ok_or(StorageError::NotFound)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Company;

    #[test]
    fn test() {
        let mut store = StubStore::new();
        let company = Company::new("Name".to_string());
        store.store(&company).unwrap();
        let stored_company = store.get_by_id(&company).unwrap();
        assert_eq!(stored_company, company);
    }
}

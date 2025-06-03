use crate::storable::property::has_id::HasId;
use crate::storage::property::base_store::BaseStore;
use crate::StorageError;

#[derive(Default)]
pub struct StubStore<T> {
    store: Vec<T>,
}

impl<T> BaseStore<T> for StubStore<T>
where
    T: HasId + Clone,
{
    async fn store(&mut self, storable: T) -> anyhow::Result<()> {
        // Remove the item if its already stored
        self.store
            .retain(|stored_item| storable.get_id() != stored_item.get_id());

        // Store the new item
        self.store.push(storable);

        Ok(())
    }

    async fn recall_by_id<I: HasId>(&self, id: I) -> anyhow::Result<T> {
        Ok(self
            .store
            .iter()
            .find(|stored_item| id.get_id() == stored_item.get_id())
            .cloned()
            .ok_or(StorageError::NotFound)?)
    }
}

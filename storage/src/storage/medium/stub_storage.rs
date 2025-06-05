use crate::storable::{Company, Flag, HasCompany, HasId, HasName, Role};
use crate::storage::{
    BaseStore, CompanyStore, FlagStore, RecallByCompany, RecallById, RecallByName, RoleStore,
};
use crate::StorageError;

pub struct StubStore<T> {
    store: Vec<T>,
}

impl<T> Default for StubStore<T> {
    fn default() -> Self {
        StubStore { store: Vec::new() }
    }
}

impl<O> BaseStore<O> for StubStore<O>
where
    O: HasId + Clone,
{
    async fn store(&mut self, storable: O) -> anyhow::Result<()> {
        // Remove the item if its already stored
        self.store
            .retain(|stored_item| storable.get_id() != stored_item.get_id());

        // Store the new item
        self.store.push(storable);

        Ok(())
    }
}

impl<O> RecallById<O> for StubStore<O>
where
    O: HasId + Clone,
{
    async fn recall_by_id<I: HasId>(&self, id: &I) -> anyhow::Result<O> {
        Ok(self
            .store
            .iter()
            .find(|stored_item| id.get_id() == stored_item.get_id())
            .cloned()
            .ok_or(StorageError::NotFound)?)
    }
}

impl<T> RecallByName<T> for StubStore<T>
where
    T: HasName + Clone,
{
    async fn recall_by_name<N: HasName>(&self, name: N) -> anyhow::Result<Vec<T>> {
        let search_string = name.get_name().to_lowercase();
        Ok(self
            .store
            .iter()
            .filter(|stored_item| {
                stored_item
                    .get_name()
                    .to_lowercase()
                    .contains(&search_string)
            })
            .cloned()
            .collect())
    }
}

impl<T> RecallByCompany<T> for StubStore<T>
where
    T: HasCompany + Clone,
{
    async fn recall_by_company<C: HasId>(&self, company: &C) -> anyhow::Result<Vec<T>> {
        Ok(self
            .store
            .iter()
            .filter(|stored_item| stored_item.get_company_id() == company.get_id())
            .cloned()
            .collect())
    }
}

impl CompanyStore for StubStore<Company> {}
impl RoleStore for StubStore<Role> {}
impl FlagStore for StubStore<Flag> {}

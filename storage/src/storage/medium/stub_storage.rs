use crate::storable::object::company::Company;
use crate::storable::object::flag::Flag;
use crate::storable::object::role::Role;
use crate::storable::property::has_company::HasCompany;
use crate::storable::property::has_id::HasId;
use crate::storable::property::has_name::HasName;
use crate::storage::object_storage::company_store::CompanyStore;
use crate::storage::object_storage::flag_store::FlagStore;
use crate::storage::object_storage::role_store::RoleStore;
use crate::storage::property::base_store::BaseStore;
use crate::storage::property::recall_by_company::RecallByCompany;
use crate::storage::property::recall_by_name::RecallByName;
use crate::StorageError;

pub struct StubStore<T> {
    store: Vec<T>,
}

impl<T> Default for StubStore<T> {
    fn default() -> Self {
        StubStore { store: Vec::new() }
    }
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

    async fn recall_by_id<I: HasId>(&self, id: &I) -> anyhow::Result<T> {
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

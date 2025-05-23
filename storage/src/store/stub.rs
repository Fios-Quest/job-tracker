use crate::error::StorageError;
use crate::store::{Store, StubStore};
use crate::utils::{GetDeleted, GetId, GetName, SetDeleted};
use crate::{utils, Timestamp};
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

impl<T> StubStore<T> {
    pub fn new() -> Self {
        Self { store: Vec::new() }
    }
}

#[async_trait]
impl<T> Store<T> for StubStore<T>
where
    T: GetName + GetId + GetDeleted + SetDeleted + Clone + Send + Sync,
{
    async fn get_by_id(&self, id: Uuid) -> Result<T> {
        self.store
            .iter()
            .filter(|t| t.get_deleted().is_none())
            .find(|t| t.get_id() == id)
            .cloned()
            .ok_or(StorageError::NotFound.into())
    }

    async fn get_by_name(&self, name: &str) -> Result<T> {
        self.store
            .iter()
            .filter(|t| t.get_deleted().is_none())
            .find(|t| t.get_name() == name)
            .cloned()
            .ok_or(StorageError::NotFound.into())
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<T>> {
        let mut results: Vec<_> = self
            .store
            .iter()
            .filter(|t| t.get_name().contains(name))
            .cloned()
            .collect();
        results.sort_by(utils::create_name_sort(name));
        Ok(results)
    }

    async fn create(&mut self, item: &T) -> Result<()> {
        // Todo: join these futures
        if self.get_by_id(item.get_id()).await.is_ok() {
            return Err(StorageError::AlreadyExists.into());
        }
        self.store.push(item.clone());
        Ok(())
    }

    async fn update(&mut self, item: &T) -> Result<()> {
        let role = self
            .store
            .iter_mut()
            .find(|role| role.get_id() == item.get_id())
            .ok_or(StorageError::NotFound)?;

        *role = item.clone();

        Ok(())
    }

    async fn delete_by_id(&mut self, id: Uuid, date_deleted: Timestamp) -> Result<()> {
        self.store
            .iter_mut()
            .filter(|c| c.get_id() == id)
            .map(|t| {
                t.set_deleted(date_deleted);
            })
            .next()
            .ok_or(StorageError::NotFound.into())
    }
}

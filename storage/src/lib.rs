use crate::utils::{GetDeleted, GetId, GetName, SetDeleted};
use async_trait::async_trait;
use uuid::Uuid;

mod company;
mod flag;
mod role;
mod time;
mod utils;

pub use company::*;
pub use flag::*;
pub use role::*;
pub use time::*;

// type SafeFuture<T> = impl std::future::Future<Output = T> + Send;

#[async_trait]
pub trait Store<T> {
    async fn get_by_id(&self, id: Uuid) -> Result<T, StorageError>;

    async fn get_by_name(&self, name: &str) -> Result<T, StorageError>;

    async fn find_by_name(&self, name: &str) -> Result<Vec<T>, StorageError>;

    async fn create(&mut self, item: T) -> Result<(), StorageError>;

    async fn delete_by_id(&mut self, id: Uuid, date_deleted: Timestamp)
        -> Result<(), StorageError>;
}

#[derive(Clone)]
pub struct StubStore<T> {
    store: Vec<T>,
}

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
    async fn get_by_id(&self, id: Uuid) -> Result<T, StorageError> {
        self.store
            .iter()
            .filter(|t| t.get_deleted().is_none())
            .find(|t| t.get_id() == id)
            .cloned()
            .ok_or(StorageError::NotFound)
    }

    async fn get_by_name(&self, name: &str) -> Result<T, StorageError> {
        self.store
            .iter()
            .filter(|t| t.get_deleted().is_none())
            .find(|t| t.get_name() == name)
            .cloned()
            .ok_or(StorageError::NotFound)
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<T>, StorageError> {
        let mut results: Vec<_> = self
            .store
            .iter()
            .filter(|t| t.get_name().contains(name))
            .cloned()
            .collect();
        results.sort_by(utils::create_name_sort(name));
        Ok(results)
    }

    async fn create(&mut self, item: T) -> Result<(), StorageError> {
        // Todo: join these futures
        if self.get_by_name(&item.get_name()).await.is_ok()
            || self.get_by_id(item.get_id()).await.is_ok()
        {
            return Err(StorageError::AlreadyExists);
        }
        self.store.push(item);
        Ok(())
    }

    async fn delete_by_id(
        &mut self,
        id: Uuid,
        date_deleted: Timestamp,
    ) -> Result<(), StorageError> {
        self.store
            .iter_mut()
            .filter(|c| c.get_id() == id)
            .map(|t| {
                t.set_deleted(date_deleted);
                () // Return Unit Type
            })
            .next()
            .ok_or(StorageError::NotFound)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StorageError {
    NotFound,
    AlreadyExists,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestStorable {
        id: Uuid,
        name: String,
        date_deleted: Option<Timestamp>,
    }

    impl TestStorable {
        fn new(name: String) -> TestStorable {
            TestStorable {
                id: Uuid::new_v4(),
                name,
                date_deleted: None,
            }
        }
    }

    impl GetName for TestStorable {
        fn get_name(&self) -> &str {
            &self.name
        }
    }

    impl GetId for TestStorable {
        fn get_id(&self) -> Uuid {
            self.id
        }
    }

    impl GetDeleted for TestStorable {
        fn get_deleted(&self) -> Option<Timestamp> {
            self.date_deleted
        }
    }

    impl SetDeleted for TestStorable {
        fn set_deleted(&mut self, time: Timestamp) {
            self.date_deleted = Some(time);
        }
    }

    #[tokio::test]
    async fn test_get_by_id() {
        let mut store = StubStore::new();
        let storable = TestStorable::new("Test".to_string());
        assert!(store.create(storable.clone()).await.is_ok());

        assert_eq!(storable.id, store.get_by_id(storable.id).await.unwrap().id);
    }

    #[tokio::test]
    async fn test_get_by_name() {
        let mut store = StubStore::new();
        let name = "Test";
        let storable = TestStorable::new(name.to_string());
        assert!(store.create(storable).await.is_ok());

        // Test can be found
        assert_eq!(name, store.get_by_name(name).await.unwrap().name);
        // Test no partial match
        assert_eq!(
            Err(StorageError::NotFound),
            store.get_by_name(&name[..1]).await
        );
    }

    #[tokio::test]
    async fn test_find_by_name() {
        let mut store = StubStore::new();
        let name = "Test";
        let storable = TestStorable::new(name.to_string());
        assert!(store.create(storable).await.is_ok());

        // Test can be found with exact match
        assert!(!store.find_by_name(name).await.unwrap().is_empty());
        // Test can be found with partial match
        assert!(!store.find_by_name(&name[..1]).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_create() {
        let mut store = StubStore::new();
        let storable = TestStorable::new("Test".to_string());

        // Should be able to create the item once
        assert!(store.create(storable.clone()).await.is_ok());
        assert_eq!(Ok(storable.clone()), store.get_by_id(storable.id).await);

        // Should not be able to store an item with the same name
        let storable_same_name = TestStorable::new("Test".to_string());
        assert_eq!(
            Err(StorageError::AlreadyExists),
            store.create(storable_same_name).await
        );

        // Should not be able to store an item with the same id
        let storable_same_id = TestStorable {
            id: storable.id,
            name: "Test".to_string(),
            date_deleted: None,
        };
        assert_eq!(
            Err(StorageError::AlreadyExists),
            store.create(storable_same_id).await
        );
    }

    #[tokio::test]
    async fn test_delete_by_id() {
        let mut store = StubStore::new();
        let storable = TestStorable::new("Test".to_string());
        assert!(store.create(storable.clone()).await.is_ok());
        assert_eq!(Ok(storable.clone()), store.get_by_id(storable.id).await);
        assert!(store
            .delete_by_id(storable.id, Timestamp::now())
            .await
            .is_ok());
        assert_eq!(
            Err(StorageError::NotFound),
            store.get_by_id(storable.id).await
        );
    }
}

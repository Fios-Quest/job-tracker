use async_trait::async_trait;
use std::marker::PhantomData;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use uuid::Uuid;

mod rocks;
mod stub;

mod company;
pub use company::*;

mod flag;
pub use flag::*;

mod role;
pub use role::*;

use crate::{GetDeleted, GetId, GetName, SetDeleted, Timestamp};

#[async_trait]
pub trait Store<T> {
    async fn get_by_id(&self, id: Uuid) -> Result<T, StorageError>;

    async fn get_by_name(&self, name: &str) -> Result<T, StorageError>;

    async fn find_by_name(&self, name: &str) -> Result<Vec<T>, StorageError>;

    async fn create(&mut self, item: T) -> Result<(), StorageError>;

    async fn delete_by_id(&mut self, id: Uuid, date_deleted: Timestamp)
        -> Result<(), StorageError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum StorageError {
    NotFound,
    AlreadyExists,
    SurrealError(String), // Not ideal, but Surreal errors are a bit weird
}

impl From<surrealdb::Error> for StorageError {
    fn from(e: surrealdb::Error) -> Self {
        StorageError::SurrealError(e.to_string())
    }
}

#[derive(Clone)]
pub struct StubStore<T> {
    store: Vec<T>,
}

impl<T> Default for StubStore<T> {
    fn default() -> Self {
        Self { store: Vec::new() }
    }
}

#[derive(Debug, Clone)]
pub struct RocksStore<T>
where
    T: GetName + GetId + GetDeleted + SetDeleted + Clone + Send + Sync,
{
    phantom_data: PhantomData<T>,
    db: Surreal<Db>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GetDeleted, GetId, GetName, SetDeleted};

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

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

mod json;
mod stub;

mod company;
pub use company::*;

mod flag;
pub use flag::*;

mod role;
use crate::Timestamp;
pub use role::*;

#[async_trait]
pub trait Store<T> {
    async fn get_by_id(&self, id: Uuid) -> Result<T>;

    async fn get_by_name(&self, name: &str) -> Result<T>;

    async fn find_by_name(&self, name: &str) -> Result<Vec<T>>;

    async fn create(&mut self, item: &T) -> Result<()>;

    async fn update(&mut self, item: &T) -> Result<()>;

    async fn delete_by_id(&mut self, id: Uuid, date_deleted: Timestamp) -> Result<()>;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GetDeleted, GetId, GetName, SetDeleted, StorageError};

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
        assert!(store.create(&storable).await.is_ok());

        assert_eq!(storable.id, store.get_by_id(storable.id).await.unwrap().id);
    }

    #[tokio::test]
    async fn test_get_by_name() {
        let mut store = StubStore::new();
        let name = "Test";
        let storable = TestStorable::new(name.to_string());
        assert!(store.create(&storable).await.is_ok());

        // Test can be found
        assert_eq!(name, store.get_by_name(name).await.unwrap().name);
        // Test no partial match
        assert!(store
            .get_by_name(&name[..1])
            .await
            .expect_err("Should error")
            .downcast::<StorageError>()
            .expect("Should be StorageError")
            .is_not_found());
    }

    #[tokio::test]
    async fn test_find_by_name() {
        let mut store = StubStore::new();
        let name = "Test";
        let storable = TestStorable::new(name.to_string());
        assert!(store.create(&storable).await.is_ok());

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
        assert!(store.create(&storable).await.is_ok());
        assert_eq!(
            store
                .get_by_id(storable.id)
                .await
                .expect("Should be created"),
            storable
        );

        // Should be able to store an item with the same name
        let storable_same_name = TestStorable::new("Test".to_string());
        assert!(store.create(&storable_same_name).await.is_ok());

        // Should not be able to store an item with the same id
        let storable_same_id = TestStorable {
            id: storable.id,
            name: "Test".to_string(),
            date_deleted: None,
        };
        assert!(store
            .create(&storable_same_id)
            .await
            .expect_err("Should already exist")
            .downcast::<StorageError>()
            .expect("Should be StorageError")
            .is_already_exists());
    }

    #[tokio::test]
    async fn test_delete_by_id() {
        let mut store = StubStore::new();
        let storable = TestStorable::new("Test".to_string());
        assert!(store.create(&storable).await.is_ok());
        assert_eq!(
            store
                .get_by_id(storable.id)
                .await
                .expect("Should be created"),
            storable
        );
        assert!(store
            .delete_by_id(storable.id, Timestamp::now())
            .await
            .is_ok());
        assert!(store
            .get_by_id(storable.id)
            .await
            .expect_err("Should not be found")
            .downcast::<StorageError>()
            .expect("Should be StorageError")
            .is_not_found());
    }
}

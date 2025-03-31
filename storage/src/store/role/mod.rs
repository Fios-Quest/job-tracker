mod stub_role_store;
pub use stub_role_store::*;

mod rocks_role_store;
pub use rocks_role_store::RocksRoleStore;

use crate::store::{StorageError, Store};
use crate::utils::{GetDeleted, GetId, GetName, SetDeleted};
use crate::Timestamp;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: Uuid,
    pub company_id: Uuid,
    pub name: String,
    pub date_applied: Timestamp,
    pub date_deleted: Option<Timestamp>,
}

impl PartialEq for Role {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name && self.company_id == other.company_id
    }
}

impl Role {
    pub fn new(company_id: Uuid, name: String, date_applied: Timestamp) -> Role {
        Role {
            id: Uuid::new_v4(),
            company_id,
            name,
            date_applied,
            date_deleted: None,
        }
    }
}

impl GetName for Role {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl GetId for Role {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl GetDeleted for Role {
    fn get_deleted(&self) -> Option<Timestamp> {
        self.date_deleted
    }
}

impl SetDeleted for Role {
    fn set_deleted(&mut self, time: Timestamp) {
        self.date_deleted = Some(time);
    }
}

#[async_trait]
pub trait RoleStore<T: Store<Role> = Self>: Store<Role> {
    async fn get_for_company(&self, id: Uuid) -> Result<Vec<Role>, StorageError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Reusable test functions
    async fn test_get_by_id<C: Store<Role>>(store: &mut C) {
        let role = Role::new(Uuid::new_v4(), "Test".to_string(), Timestamp::now());
        assert!(store.create(role.clone()).await.is_ok());

        assert_eq!(role.id, store.get_by_id(role.id).await.unwrap().id);
    }

    async fn test_get_by_name<C: Store<Role>>(store: &mut C) {
        let name = "Test";
        let role = Role::new(Uuid::new_v4(), name.to_string(), Timestamp::now());
        assert!(store.create(role).await.is_ok());

        // Test can be found
        assert_eq!(name, store.get_by_name(name).await.unwrap().name);
        // Test no partial match
        assert_eq!(
            Err(StorageError::NotFound),
            store.get_by_name(&name[..1]).await
        );
    }
    async fn test_find_by_name<C: Store<Role>>(store: &mut C) {
        let name = "Test";
        let role = Role::new(Uuid::new_v4(), name.to_string(), Timestamp::now());
        assert!(store.create(role).await.is_ok());

        // Test can be found with exact match
        assert!(!store.find_by_name(name).await.unwrap().is_empty());
        // Test can be found with partial match
        assert!(!store.find_by_name(&name[..1]).await.unwrap().is_empty());
    }
    async fn test_create<C: Store<Role>>(store: &mut C) {
        let role = Role::new(Uuid::new_v4(), "Test".to_string(), Timestamp::now());

        // Should be able to create the role once
        assert!(store.create(role.clone()).await.is_ok());
        assert_eq!(Ok(role.clone()), store.get_by_id(role.id).await);

        // Should not be able to store a role with the same name
        let role_same_name = Role::new(Uuid::new_v4(), "Test".to_string(), Timestamp::now());
        assert_eq!(
            Err(StorageError::AlreadyExists),
            store.create(role_same_name).await
        );

        // Should not be able to store a role with the same id
        let role_same_id = Role {
            id: role.id,
            company_id: role.company_id,
            name: "Test".to_string(),
            date_applied: Timestamp::now(),
            date_deleted: None,
        };
        assert_eq!(
            Err(StorageError::AlreadyExists),
            store.create(role_same_id).await
        );
    }
    async fn test_delete_by_id<C: Store<Role>>(store: &mut C) {
        let role = Role::new(Uuid::new_v4(), "Test".to_string(), Timestamp::now());
        assert!(store.create(role.clone()).await.is_ok());
        assert_eq!(Ok(role.clone()), store.get_by_id(role.id).await);
        assert!(store.delete_by_id(role.id, Timestamp::now()).await.is_ok());
        assert_eq!(Err(StorageError::NotFound), store.get_by_id(role.id).await);
    }

    async fn test_get_for_company<C: RoleStore>(store: &mut C) {
        let company1 = Uuid::new_v4();
        let company2 = Uuid::new_v4();
        let role1 = Role::new(company1, "Test 1".to_string(), Timestamp::now());
        let role2 = Role::new(company1, "Test 2".to_string(), Timestamp::now());
        let role3 = Role::new(company2, "Test 3".to_string(), Timestamp::now());
        let role4 = Role::new(company2, "Test 4".to_string(), Timestamp::now());
        assert!(store.create(role1.clone()).await.is_ok());
        assert!(store.create(role2.clone()).await.is_ok());
        assert!(store.create(role3.clone()).await.is_ok());
        assert!(store.create(role4.clone()).await.is_ok());
        assert_eq!(
            Ok(vec![role1, role2]),
            store.get_for_company(company1).await
        );
        assert_eq!(
            Ok(vec![role3, role4]),
            store.get_for_company(company2).await
        );
    }

    // Module for each implementation
    mod stub_role_store {
        use crate::store::role::stub_role_store::StubRoleStore;

        #[tokio::test]
        async fn test_get_by_id() {
            let mut store = StubRoleStore::new();
            super::test_get_by_id(&mut store).await;
        }

        #[tokio::test]
        async fn test_get_by_name() {
            let mut store = StubRoleStore::new();
            super::test_get_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_find_by_name() {
            let mut store = StubRoleStore::new();
            super::test_find_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_create_role() {
            let mut store = StubRoleStore::new();
            super::test_create(&mut store).await;
        }

        #[tokio::test]
        async fn test_delete_by_id() {
            let mut store = StubRoleStore::new();
            super::test_delete_by_id(&mut store).await;
        }

        #[tokio::test]
        async fn get_for_company() {
            let mut store = StubRoleStore::new();
            super::test_get_for_company(&mut store).await;
        }
    }

    mod rocks_role_store {
        use crate::RocksRoleStore;
        use tempdir::TempDir;

        #[tokio::test]
        async fn test_get_by_id() {
            let tmp_dir = TempDir::new("role_test").unwrap();
            let mut store = RocksRoleStore::new_from_path(tmp_dir.as_ref())
                .await
                .unwrap();
            super::test_get_by_id(&mut store).await;
        }

        #[tokio::test]
        async fn test_get_by_name() {
            let tmp_dir = TempDir::new("role_test").unwrap();
            let mut store = RocksRoleStore::new_from_path(tmp_dir.as_ref())
                .await
                .unwrap();
            super::test_get_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_find_by_name() {
            let tmp_dir = TempDir::new("role_test").unwrap();
            let mut store = RocksRoleStore::new_from_path(tmp_dir.as_ref())
                .await
                .unwrap();
            super::test_find_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_create_role() {
            let tmp_dir = TempDir::new("role_test").unwrap();
            let mut store = RocksRoleStore::new_from_path(tmp_dir.as_ref())
                .await
                .unwrap();
            super::test_create(&mut store).await;
        }

        #[tokio::test]
        async fn test_delete_by_id() {
            let tmp_dir = TempDir::new("role_test").unwrap();
            let mut store = RocksRoleStore::new_from_path(tmp_dir.as_ref())
                .await
                .unwrap();
            super::test_delete_by_id(&mut store).await;
        }

        #[tokio::test]
        async fn get_for_company() {
            let tmp_dir = TempDir::new("role_test").unwrap();
            let mut store = RocksRoleStore::new_from_path(tmp_dir.as_ref())
                .await
                .unwrap();
            super::test_get_for_company(&mut store).await;
        }
    }

    mod role {
        use super::*;

        #[test]
        fn test_get_name() {
            let company = Role::new(Uuid::new_v4(), "Test".to_string(), Timestamp::now());
            assert_eq!(company.get_name(), company.name);
        }

        #[test]
        fn test_get_id() {
            let company = Role::new(Uuid::new_v4(), "Test".to_string(), Timestamp::now());
            assert_eq!(company.get_id(), company.id);
        }

        #[test]
        fn test_get_and_set_deleted() {
            let mut company = Role::new(Uuid::new_v4(), "Test".to_string(), Timestamp::now());
            assert_eq!(company.get_deleted(), None);
            let time = Timestamp::now();
            company.set_deleted(time);
            assert_eq!(company.get_deleted(), Some(time));
        }
    }
}

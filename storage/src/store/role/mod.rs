mod stub_role_store;
pub use stub_role_store::StubRoleStore;

mod json_role_store;
pub use json_role_store::JsonRoleStore;

use crate::store::{StorageError, Store};
use crate::utils::{GetDeleted, GetId, GetName, SetDeleted};
use crate::{GetDescription, SetDescription, Timestamp};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: Uuid,
    pub company_id: Uuid,
    pub name: String,
    pub description: String,
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
            description: "".to_string(),
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

impl GetDescription for Role {
    fn get_description(&self) -> &String {
        &self.description
    }
}

impl SetDescription for Role {
    fn set_description(&mut self, value: String) {
        self.description = value;
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
        assert_eq!(store.create(role.clone()).await, Ok(()));

        assert_eq!(role.id, store.get_by_id(role.id).await.unwrap().id);
    }

    async fn test_get_by_name<C: Store<Role>>(store: &mut C) {
        let name = "Test";
        let role = Role::new(Uuid::new_v4(), name.to_string(), Timestamp::now());
        assert_eq!(store.create(role).await, Ok(()));

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
        let t_role = Role::new(Uuid::new_v4(), name.to_string(), Timestamp::now());
        assert_eq!(store.create(t_role.clone()).await, Ok(()));

        // Test can be found with exact match
        assert!(!store.find_by_name(name).await.unwrap().is_empty());
        // Test can be found with partial match
        assert!(!store.find_by_name(&name[..1]).await.unwrap().is_empty());

        // It should return all companies when search string is empty
        let a_role = Role::new(Uuid::new_v4(), "Another role".to_string(), Timestamp::now());
        let y_role = Role::new(
            Uuid::new_v4(),
            "Yet Another role".to_string(),
            Timestamp::now(),
        );
        assert_eq!(store.create(a_role.clone()).await, Ok(()));
        assert_eq!(store.create(y_role.clone()).await, Ok(()));
        assert_eq!(
            store.find_by_name("").await,
            Ok(vec![a_role, t_role, y_role])
        );
    }
    async fn test_create<C: Store<Role>>(store: &mut C) {
        let role = Role::new(Uuid::new_v4(), "Test".to_string(), Timestamp::now());

        // Should be able to create the role once
        assert_eq!(store.create(role.clone()).await, Ok(()));
        assert_eq!(Ok(role.clone()), store.get_by_id(role.id).await);

        // Should be able to store a role with the same name
        let role_same_name = Role::new(Uuid::new_v4(), "Test".to_string(), Timestamp::now());
        assert!(store.create(role_same_name).await.is_ok());

        // Should not be able to store a role with the same id
        let role_same_id = Role {
            id: role.id,
            company_id: role.company_id,
            name: "Test".to_string(),
            description: "".to_string(),
            date_applied: Timestamp::now(),
            date_deleted: None,
        };
        assert_eq!(
            Err(StorageError::AlreadyExists),
            store.create(role_same_id).await
        );
    }

    async fn test_update<C: Store<Role>>(store: &mut C) {
        let mut role = Role::new(Uuid::new_v4(), "Test".to_string(), Timestamp::now());
        assert_eq!(store.create(role.clone()).await, Ok(()));
        assert_eq!(Ok(role.clone()), store.get_by_id(role.id).await);
        role.description = "This is a description".to_string();
        assert_eq!(store.update(role.clone()).await, Ok(()));
        assert_eq!(store.get_by_id(role.id).await, Ok(role));
    }

    async fn test_delete_by_id<C: Store<Role>>(store: &mut C) {
        let role = Role::new(Uuid::new_v4(), "Test".to_string(), Timestamp::now());
        assert_eq!(store.create(role.clone()).await, Ok(()));
        assert_eq!(Ok(role.clone()), store.get_by_id(role.id).await);
        assert_eq!(store.delete_by_id(role.id, Timestamp::now()).await, Ok(()));
        assert_eq!(Err(StorageError::NotFound), store.get_by_id(role.id).await);
    }

    async fn test_get_for_company<C: RoleStore>(store: &mut C) {
        let company1 = Uuid::new_v4();
        let company2 = Uuid::new_v4();
        let role1 = Role::new(company1, "Test 1".to_string(), Timestamp::now());
        let role2 = Role::new(company1, "Test 2".to_string(), Timestamp::now());
        let role3 = Role::new(company2, "Test 3".to_string(), Timestamp::now());
        let role4 = Role::new(company2, "Test 4".to_string(), Timestamp::now());
        assert_eq!(store.create(role1.clone()).await, Ok(()));
        assert_eq!(store.create(role2.clone()).await, Ok(()));
        assert_eq!(store.create(role3.clone()).await, Ok(()));
        assert_eq!(store.create(role4.clone()).await, Ok(()));
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
        async fn test_update_role() {
            let mut store = StubRoleStore::new();
            super::test_update(&mut store).await;
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

    mod json_role_store {
        use crate::store::role::json_role_store::JsonRoleStore;

        #[tokio::test]
        async fn test_get_by_id() {
            let mut store = JsonRoleStore::new_tmp().await.unwrap();
            super::test_get_by_id(&mut store).await;
        }

        #[tokio::test]
        async fn test_get_by_name() {
            let mut store = JsonRoleStore::new_tmp().await.unwrap();
            super::test_get_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_find_by_name() {
            let mut store = JsonRoleStore::new_tmp().await.unwrap();
            super::test_find_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_create_role() {
            let mut store = JsonRoleStore::new_tmp().await.unwrap();
            super::test_create(&mut store).await;
        }

        #[tokio::test]
        async fn test_update_role() {
            let mut store = JsonRoleStore::new_tmp().await.unwrap();
            super::test_update(&mut store).await;
        }

        #[tokio::test]
        async fn test_delete_by_id() {
            let mut store = JsonRoleStore::new_tmp().await.unwrap();
            super::test_delete_by_id(&mut store).await;
        }

        #[tokio::test]
        async fn get_for_company() {
            let mut store = JsonRoleStore::new_tmp().await.unwrap();
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

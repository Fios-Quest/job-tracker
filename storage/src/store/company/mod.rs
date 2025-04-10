mod stub_company_store;
pub use stub_company_store::StubCompanyStore;

mod rocks_company_store;
pub use rocks_company_store::RocksCompanyStore;

mod libsql_company_store;
pub use libsql_company_store::LibSqlCompanyStore;

use crate::utils::{GetDeleted, GetId, GetName, SetDeleted};
use crate::{Role, Timestamp};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub date_deleted: Option<Timestamp>,
}

impl PartialEq for Company {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

impl GetName for Company {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl GetId for Company {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl GetDeleted for Company {
    fn get_deleted(&self) -> Option<Timestamp> {
        self.date_deleted
    }
}

impl SetDeleted for Company {
    fn set_deleted(&mut self, time: Timestamp) {
        self.date_deleted = Some(time);
    }
}

impl Company {
    pub fn new(name: String) -> Company {
        Company {
            id: Uuid::new_v4(),
            name,
            date_deleted: None,
        }
    }

    pub fn create_role(&self, name: String, date_created: Timestamp) -> Role {
        Role::new(self.id, name, date_created)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::{StorageError, Store};

    // Reusable test functions
    async fn test_get_by_id<C: Store<Company>>(store: &mut C) {
        let company = Company::new("Test".to_string());
        assert_eq!(store.create(company.clone()).await, Ok(()));

        assert_eq!(store.get_by_id(company.id).await.unwrap().id, company.id);
    }

    async fn test_get_by_name<C: Store<Company>>(store: &mut C) {
        let name = "Test";
        let company = Company::new(name.to_string());
        assert_eq!(store.create(company).await, Ok(()));

        // Test can be found
        let result = store.get_by_name(name).await;

        assert_eq!(result.unwrap().name, name);
        // Test no partial match
        assert_eq!(
            store.get_by_name(&name[..1]).await,
            Err(StorageError::NotFound)
        );
    }

    async fn test_find_by_name<C: Store<Company>>(store: &mut C) {
        let name = "Test";
        let t_company = Company::new(name.to_string());
        assert_eq!(store.create(t_company.clone()).await, Ok(()));

        // Test can be found with exact match
        assert!(!store.find_by_name(name).await.unwrap().is_empty());
        // Test can be found with partial match
        assert!(!store.find_by_name(&name[..1]).await.unwrap().is_empty());

        // It should return all companies when search string is empty
        let a_company = Company::new("Another company".to_string());
        let y_company = Company::new("Yet Another company".to_string());
        assert_eq!(store.create(a_company.clone()).await, Ok(()));
        assert_eq!(store.create(y_company.clone()).await, Ok(()));
        assert_eq!(
            store.find_by_name("").await,
            Ok(vec![a_company, t_company, y_company])
        );
    }

    async fn test_create_company<C: Store<Company>>(store: &mut C) {
        let company = Company::new("Test".to_string());

        // Should be able to create the company once
        assert_eq!(store.create(company.clone()).await, Ok(()));
        assert_eq!(store.get_by_id(company.id).await, Ok(company.clone()));

        // Should not be able to store a company with the same name
        let company_same_name = Company::new("Test".to_string());
        assert_eq!(
            store.create(company_same_name).await,
            Err(StorageError::AlreadyExists)
        );

        // Should not be able to store a company with the same id
        let company_same_id = Company {
            id: company.id,
            name: "Test".to_string(),
            date_deleted: None,
        };
        assert_eq!(
            store.create(company_same_id).await,
            Err(StorageError::AlreadyExists)
        );
    }

    async fn test_delete_by_id<C: Store<Company>>(store: &mut C) {
        let company = Company::new("Test".to_string());
        assert_eq!(store.create(company.clone()).await, Ok(()));
        assert_eq!(store.get_by_id(company.id).await, Ok(company.clone()));
        assert_eq!(
            store.delete_by_id(company.id, Timestamp::now()).await,
            Ok(())
        );
        assert_eq!(
            store.get_by_id(company.id).await,
            Err(StorageError::NotFound)
        );
    }

    // Module for each implementation
    mod stub_company_store {
        use crate::StubCompanyStore;

        #[tokio::test]
        async fn test_get_by_id() {
            let mut store = StubCompanyStore::new();
            super::test_get_by_id(&mut store).await;
        }

        #[tokio::test]
        async fn test_get_by_name() {
            let mut store = StubCompanyStore::new();
            super::test_get_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_find_by_name() {
            let mut store = StubCompanyStore::new();
            super::test_find_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_create_company() {
            let mut store = StubCompanyStore::new();
            super::test_create_company(&mut store).await;
        }

        #[tokio::test]
        async fn test_delete_by_id() {
            let mut store = StubCompanyStore::new();
            super::test_delete_by_id(&mut store).await;
        }
    }

    mod rocks_company_store {
        use crate::RocksCompanyStore;

        #[tokio::test]
        async fn test_get_by_id() {
            let mut store = RocksCompanyStore::new_tmp().await.unwrap();
            super::test_get_by_id(&mut store).await;
        }

        #[tokio::test]
        async fn test_get_by_name() {
            let mut store = RocksCompanyStore::new_tmp().await.unwrap();
            super::test_get_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_find_by_name() {
            let mut store = RocksCompanyStore::new_tmp().await.unwrap();
            super::test_find_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_create_company() {
            let mut store = RocksCompanyStore::new_tmp().await.unwrap();
            super::test_create_company(&mut store).await;
        }

        #[tokio::test]
        async fn test_delete_by_id() {
            let mut store = RocksCompanyStore::new_tmp().await.unwrap();
            super::test_delete_by_id(&mut store).await;
        }
    }

    mod libsql_company_store {
        use crate::LibSqlStore;

        #[tokio::test]
        async fn test_get_by_id() {
            let mut store = LibSqlStore::new_tmp().await.unwrap();
            super::test_get_by_id(&mut store).await;
        }

        #[tokio::test]
        async fn test_get_by_name() {
            let mut store = LibSqlStore::new_tmp().await.unwrap();
            super::test_get_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_find_by_name() {
            let mut store = LibSqlStore::new_tmp().await.unwrap();
            super::test_find_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_create_company() {
            let mut store = LibSqlStore::new_tmp().await.unwrap();
            super::test_create_company(&mut store).await;
        }

        #[tokio::test]
        async fn test_delete_by_id() {
            let mut store = LibSqlStore::new_tmp().await.unwrap();
            super::test_delete_by_id(&mut store).await;
        }
    }

    mod company {
        use super::*;

        #[test]
        fn test_get_name() {
            let company = Company::new("Test".to_string());
            assert_eq!(company.get_name(), company.name);
        }

        #[test]
        fn test_get_id() {
            let company = Company::new("Test".to_string());
            assert_eq!(company.get_id(), company.id);
        }

        #[test]
        fn test_get_and_set_deleted() {
            let mut company = Company::new("Test".to_string());
            assert_eq!(company.get_deleted(), None);
            let time = Timestamp::now();
            company.set_deleted(time);
            assert_eq!(company.get_deleted(), Some(time));
        }

        #[test]
        fn test_create_role() {
            let company = Company::new("Test".to_string());
            let role = company.create_role("Test Role".to_string(), Timestamp::now());
            assert_eq!(role.company_id, company.id);
        }
    }
}

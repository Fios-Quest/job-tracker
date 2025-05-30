mod stub_company_store;
pub use stub_company_store::StubCompanyStore;

mod json_company_store;
pub use json_company_store::JsonCompanyStore;

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
    use crate::error::StorageError;
    use crate::store::Store;

    // Reusable test functions
    async fn test_get_by_id<C: Store<Company>>(store: &mut C) {
        let company = Company::new("Test".to_string());
        assert!(store.create(&company).await.is_ok());

        assert_eq!(store.get_by_id(company.id).await.unwrap().id, company.id);
    }

    async fn test_get_by_name<C: Store<Company>>(store: &mut C) {
        let name = "Test";
        let company = Company::new(name.to_string());
        assert!(store.create(&company).await.is_ok());

        // Test can be found
        let result = store.get_by_name(name).await;

        assert_eq!(result.unwrap().name, name);
        // Test no partial match
        assert!(store
            .get_by_name(&name[..1])
            .await
            .expect_err("Should be error")
            .downcast::<StorageError>()
            .expect("Should be StorageError")
            .is_not_found());
    }

    async fn test_find_by_name<C: Store<Company>>(store: &mut C) {
        let name = "Test";
        let t_company = Company::new(name.to_string());
        assert!(store.create(&t_company).await.is_ok());

        // Test can be found with exact match
        assert!(!store.find_by_name(name).await.unwrap().is_empty());
        // Test can be found with partial match
        assert!(!store.find_by_name(&name[..1]).await.unwrap().is_empty());

        // It should return all companies when search string is empty
        let a_company = Company::new("Another company".to_string());
        let y_company = Company::new("Yet Another company".to_string());
        assert!(store.create(&a_company).await.is_ok());
        assert!(store.create(&y_company).await.is_ok());
        assert_eq!(
            store.find_by_name("").await.expect("Should exist"),
            vec![a_company, t_company, y_company]
        );
    }

    async fn test_create_company<C: Store<Company>>(store: &mut C) {
        let company = Company::new("Test".to_string());

        // Should be able to create the company once
        assert!(store.create(&company).await.is_ok());
        assert_eq!(
            store
                .get_by_id(company.id)
                .await
                .expect("Should have been created"),
            company
        );

        // Should be able to store a company with the same name
        let company_same_name = Company::new("Test".to_string());
        assert!(store.create(&company_same_name).await.is_ok());

        // Should not be able to store a company with the same id
        let company_same_id = Company {
            id: company.id,
            name: "Test".to_string(),
            date_deleted: None,
        };
        assert!(store
            .create(&company_same_id)
            .await
            .expect_err("Should be error")
            .downcast::<StorageError>()
            .expect("Should be StorageError")
            .is_already_exists(),);
    }

    async fn test_update_company<C: Store<Company>>(store: &mut C) {
        let mut company = Company::new("Test".to_string());
        assert!(store.create(&company).await.is_ok());
        assert_eq!(
            store
                .get_by_id(company.id)
                .await
                .expect("Should be created"),
            company
        );

        company.name = "Updated Name".to_string();
        assert!(store.update(&company).await.is_ok());
        assert_eq!(
            store
                .get_by_id(company.id)
                .await
                .expect("Should be updated"),
            company
        );
    }

    async fn test_delete_by_id<C: Store<Company>>(store: &mut C) {
        let company = Company::new("Test".to_string());
        assert!(store.create(&company).await.is_ok());
        assert_eq!(
            store
                .get_by_id(company.id)
                .await
                .expect("Should be created"),
            company
        );

        assert!(store
            .delete_by_id(company.id, Timestamp::now())
            .await
            .is_ok());
        assert!(store
            .get_by_id(company.id)
            .await
            .expect_err("Should be error")
            .downcast::<StorageError>()
            .expect("Should be StorageError")
            .is_not_found());
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
        async fn test_update_company() {
            let mut store = StubCompanyStore::new();
            super::test_update_company(&mut store).await;
        }

        #[tokio::test]
        async fn test_delete_by_id() {
            let mut store = StubCompanyStore::new();
            super::test_delete_by_id(&mut store).await;
        }
    }

    mod json_company_store {
        use crate::JsonCompanyStore;

        #[tokio::test]
        async fn test_get_by_id() {
            let mut store = JsonCompanyStore::new_tmp().await.unwrap();
            super::test_get_by_id(&mut store).await;
        }

        #[tokio::test]
        async fn test_get_by_name() {
            let mut store = JsonCompanyStore::new_tmp().await.unwrap();
            super::test_get_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_find_by_name() {
            let mut store = JsonCompanyStore::new_tmp().await.unwrap();
            super::test_find_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_create_company() {
            let mut store = JsonCompanyStore::new_tmp().await.unwrap();
            super::test_create_company(&mut store).await;
        }

        #[tokio::test]
        async fn test_update_company() {
            let mut store = JsonCompanyStore::new_tmp().await.unwrap();
            super::test_update_company(&mut store).await;
        }

        #[tokio::test]
        async fn test_delete_by_id() {
            let mut store = JsonCompanyStore::new_tmp().await.unwrap();
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

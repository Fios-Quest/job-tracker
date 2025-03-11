use super::StorageError;
use std::cmp::Ordering;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Company {
    id: Uuid,
    pub name: String,
    deleted: Option<SystemTime>,
}

impl PartialEq for Company {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

impl PartialOrd for Company {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Company {
    pub fn new(name: String) -> Company {
        Company {
            id: Uuid::new_v4(),
            name,
            deleted: None,
        }
    }
}

pub trait CompanyStore {
    async fn get_company_by_id(&self, id: Uuid) -> Result<Company, StorageError>;

    async fn get_company_by_name(&self, name: &str) -> Result<Company, StorageError>;

    async fn find_by_name(&self, name: &str) -> Result<Vec<Company>, StorageError>;

    async fn create_company(&mut self, company: Company) -> Result<(), StorageError>;

    async fn delete_company(&mut self, id: Uuid) -> Result<(), StorageError>;
}

struct StubCompanyStore {
    companies: Vec<Company>,
}

impl StubCompanyStore {
    pub fn new() -> StubCompanyStore {
        StubCompanyStore {
            companies: Vec::new(),
        }
    }
}

impl CompanyStore for StubCompanyStore {
    async fn get_company_by_id(&self, id: Uuid) -> Result<Company, StorageError> {
        self.companies
            .iter()
            .filter(|c| c.deleted.is_none())
            .find(|c| c.id == id)
            .cloned()
            .ok_or(StorageError::NotFound)
    }

    async fn get_company_by_name(&self, name: &str) -> Result<Company, StorageError> {
        self.companies
            .iter()
            .filter(|c| c.deleted.is_none())
            .find(|c| c.name == name)
            .cloned()
            .ok_or(StorageError::NotFound)
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<Company>, StorageError> {
        Ok(self
            .companies
            .iter()
            .filter(|c| c.name.contains(name))
            .cloned()
            .collect())
    }

    async fn create_company(&mut self, company: Company) -> Result<(), StorageError> {
        // Todo: join these futures
        if self.get_company_by_name(&company.name).await.is_ok()
            && self.get_company_by_id(company.id).await.is_ok()
        {
            return Err(StorageError::AlreadyExists);
        }
        self.companies.push(company);
        Ok(())
    }

    async fn delete_company(&mut self, id: Uuid) -> Result<(), StorageError> {
        self.companies
            .iter_mut()
            .filter(|c| c.id == id)
            .map(|c| {
                c.deleted = Some(SystemTime::now());
                () // Return Unit Type
            })
            .next()
            .ok_or(StorageError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Reusable test functions
    async fn test_get_company_by_id<C: CompanyStore>(store: &mut C) {
        let company = Company::new("Test".to_string());
        assert!(store.create_company(company.clone()).await.is_ok());

        assert_eq!(
            company.id,
            store.get_company_by_id(company.id).await.unwrap().id
        );
    }

    async fn test_get_company_by_name<C: CompanyStore>(store: &mut C) {
        let name = "Test";
        let company = Company::new(name.to_string());
        assert!(store.create_company(company).await.is_ok());

        // Test can be found
        assert_eq!(name, store.get_company_by_name(name).await.unwrap().name);
        // Test no partial match
        assert_eq!(
            Err(StorageError::NotFound),
            store.get_company_by_name(&name[..1]).await
        );
    }
    async fn test_find_by_name<C: CompanyStore>(store: &mut C) {
        let name = "Test";
        let company = Company::new(name.to_string());
        assert!(store.create_company(company).await.is_ok());

        // Test can be found with exact match
        assert!(!store.find_by_name(name).await.unwrap().is_empty());
        // Test can be found with partial match
        assert!(!store.find_by_name(&name[..1]).await.unwrap().is_empty());
    }
    async fn test_create_company<C: CompanyStore>(store: &mut C) {
        let company = Company::new("Test".to_string());
        assert!(store.create_company(company.clone()).await.is_ok());
        assert_eq!(
            Ok(company.clone()),
            store.get_company_by_id(company.id).await
        );
    }
    async fn test_delete_company<C: CompanyStore>(store: &mut C) {
        let company = Company::new("Test".to_string());
        assert!(store.create_company(company.clone()).await.is_ok());
        assert_eq!(
            Ok(company.clone()),
            store.get_company_by_id(company.id).await
        );
        assert!(store.delete_company(company.id).await.is_ok());
        assert_eq!(
            Err(StorageError::NotFound),
            store.get_company_by_id(company.id).await
        );
    }

    // Module for each implementation
    mod stub_company_store {
        use super::*;

        #[tokio::test]
        async fn test_get_company_by_id() {
            let mut store = StubCompanyStore::new();
            super::test_get_company_by_id(&mut store).await;
        }

        #[tokio::test]
        async fn test_get_company_by_name() {
            let mut store = StubCompanyStore::new();
            super::test_get_company_by_name(&mut store).await;
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
        async fn test_delete_company() {
            let mut store = StubCompanyStore::new();
            super::test_delete_company(&mut store).await;
        }
    }
}

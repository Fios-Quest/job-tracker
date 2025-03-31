use crate::stores::Stores;
use crate::{StubCompanyStore, StubFlagStore, StubRoleStore};

pub struct StubStores {
    company_store: StubCompanyStore,
    role_store: StubRoleStore,
    flag_store: StubFlagStore,
}

impl StubStores {
    pub fn new() -> Self {
        Self {
            company_store: StubCompanyStore::new(),
            role_store: StubRoleStore::new(),
            flag_store: StubFlagStore::new(),
        }
    }
}

impl Stores<StubCompanyStore, StubRoleStore, StubFlagStore> for StubStores {
    fn company_store(&mut self) -> &mut StubCompanyStore {
        &mut self.company_store
    }

    fn role_store(&mut self) -> &mut StubRoleStore {
        &mut self.role_store
    }

    fn flag_store(&mut self) -> &mut StubFlagStore {
        &mut self.flag_store
    }
}

#[cfg(test)]
mod tests {
    use crate::stores::stub_stores::StubStores;
    use crate::stores::*;
    use crate::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_company_store() {
        let mut stores = StubStores::new();

        let company = Company::new("Test Company".to_string());
        stores
            .company_store()
            .create(company.clone())
            .await
            .unwrap();
        assert_eq!(
            stores
                .company_store()
                .get_by_id(company.get_id())
                .await
                .unwrap(),
            company
        );
    }

    #[tokio::test]
    async fn test_role_store() {
        let mut stores = StubStores::new();

        let role = Role::new(Uuid::new_v4(), "Test Role".to_string(), Timestamp::now());
        stores.role_store().create(role.clone()).await.unwrap();
        assert_eq!(
            stores.role_store().get_by_id(role.get_id()).await.unwrap(),
            role
        );
    }

    #[tokio::test]
    async fn test_flag_store() {
        let mut stores = StubStores::new();

        let flag = Flag::new_green(Uuid::new_v4(), "Test Flag".to_string());
        stores.flag_store().create(flag.clone()).await.unwrap();
        assert_eq!(
            stores.flag_store().get_by_id(flag.get_id()).await.unwrap(),
            flag
        );
    }
}

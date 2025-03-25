use crate::{Company, FlagStore, RoleStore, Store};

pub struct Stores<C, R, F>
where
    C: Store<Company>,
    R: RoleStore,
    F: FlagStore,
{
    company_store: C,
    role_store: R,
    flag_store: F,
}

impl<C, R, F> Stores<C, R, F>
where
    C: Store<Company>,
    R: RoleStore,
    F: FlagStore,
{
    pub fn new(company_store: C, role_store: R, flag_store: F) -> Self {
        Self {
            company_store,
            role_store,
            flag_store,
        }
    }

    pub fn company_store(&mut self) -> &mut C {
        &mut self.company_store
    }

    pub fn role_store(&mut self) -> &mut R {
        &mut self.role_store
    }

    pub fn flag_store(&mut self) -> &mut F {
        &mut self.flag_store
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_company_store() {
        let company_store = StubCompanyStore::new();
        let role_store = StubRoleStore::new();
        let flag_store = StubFlagStore::new();
        let mut stores = Stores::new(company_store, role_store, flag_store);

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
        let company_store = StubCompanyStore::new();
        let role_store = StubRoleStore::new();
        let flag_store = StubFlagStore::new();
        let mut stores = Stores::new(company_store, role_store, flag_store);

        let role = Role::new(Uuid::new_v4(), "Test Role".to_string(), Timestamp::now());
        stores.role_store().create(role.clone()).await.unwrap();
        assert_eq!(
            stores.role_store().get_by_id(role.get_id()).await.unwrap(),
            role
        );
    }

    #[tokio::test]
    async fn test_flag_store() {
        let company_store = StubCompanyStore::new();
        let role_store = StubRoleStore::new();
        let flag_store = StubFlagStore::new();
        let mut stores = Stores::new(company_store, role_store, flag_store);

        let flag = Flag::new_green(Uuid::new_v4(), "Test Flag".to_string());
        stores.flag_store().create(flag.clone()).await.unwrap();
        assert_eq!(
            stores.flag_store().get_by_id(flag.get_id()).await.unwrap(),
            flag
        );
    }
}

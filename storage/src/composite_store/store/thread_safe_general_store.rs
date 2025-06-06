use crate::composite_store::HasFutureStoreFor;
use crate::storable::{Company, Flag, Role};
use crate::storage::{CompanyStore, FlagStore, RoleStore};
use crate::Sealed;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Clone)]
pub struct ThreadSafeGeneralStore<C, F, R>
where
    C: CompanyStore,
    F: FlagStore,
    R: RoleStore,
{
    company_store: Arc<Mutex<C>>,
    flag_store: Arc<Mutex<F>>,
    role_store: Arc<Mutex<R>>,
}

impl<C, F, R> ThreadSafeGeneralStore<C, F, R>
where
    C: CompanyStore,
    F: FlagStore,
    R: RoleStore,
{
    pub fn new(company_store: C, flag_store: F, role_store: R) -> Self {
        Self {
            company_store: Arc::new(Mutex::new(company_store)),
            flag_store: Arc::new(Mutex::new(flag_store)),
            role_store: Arc::new(Mutex::new(role_store)),
        }
    }

    pub async fn company_store(&self) -> MutexGuard<C> {
        self.company_store.lock().await
    }

    pub async fn flag_store(&self) -> MutexGuard<F> {
        self.flag_store.lock().await
    }

    pub async fn role_store(&self) -> MutexGuard<R> {
        self.role_store.lock().await
    }
}

impl<C, F, R> Sealed for ThreadSafeGeneralStore<C, F, R>
where
    C: CompanyStore,
    F: FlagStore,
    R: RoleStore,
{
}

impl<C, F, R> HasFutureStoreFor<Company> for ThreadSafeGeneralStore<C, F, R>
where
    C: CompanyStore,
    F: FlagStore,
    R: RoleStore,
{
    type Storage = C;

    async fn get_store<'a>(&'a self) -> MutexGuard<'a, Self::Storage>
    where
        Self::Storage: 'a,
    {
        self.company_store().await
    }
}

impl<C, F, R> HasFutureStoreFor<Flag> for ThreadSafeGeneralStore<C, F, R>
where
    C: CompanyStore,
    F: FlagStore,
    R: RoleStore,
{
    type Storage = F;

    async fn get_store<'a>(&'a self) -> MutexGuard<'a, Self::Storage>
    where
        Self::Storage: 'a,
    {
        self.flag_store().await
    }
}

impl<C, F, R> HasFutureStoreFor<Role> for ThreadSafeGeneralStore<C, F, R>
where
    C: CompanyStore,
    F: FlagStore,
    R: RoleStore,
{
    type Storage = R;

    async fn get_store<'a>(&'a self) -> MutexGuard<'a, Self::Storage>
    where
        Self::Storage: 'a,
    {
        self.role_store().await
    }
}

#[cfg(test)]
mod test_helper {
    use super::*;
    use crate::storage::StubStore;
    use crate::test_helper::TestHelper;

    #[cfg(test)]
    impl TestHelper for ThreadSafeGeneralStore<StubStore<Company>, StubStore<Flag>, StubStore<Role>> {
        #[cfg(test)]
        async fn new_test() -> anyhow::Result<Self> {
            let store = ThreadSafeGeneralStore::new(
                StubStore::default(),
                StubStore::default(),
                StubStore::default(),
            );
            Ok(store)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::composite_store::ThreadSafeGeneralStore;
    use crate::prelude::*;
    use crate::storage::recall_by_id::test_helper::test_recall_by_id;
    use crate::storage::{
        recall_by_company::test_helper::test_recall_by_company,
        recall_by_name::test_helper::test_recall_by_name, StubStore,
    };
    use crate::test_helper::*;
    use crate::Timestamp;
    use paste::paste;

    test_recall_by_id!(ThreadSafeGeneralStore, Company);
    test_recall_by_id!(ThreadSafeGeneralStore, Flag);
    test_recall_by_id!(ThreadSafeGeneralStore, Role);
    test_recall_by_name!(ThreadSafeGeneralStore, Company);
    test_recall_by_name!(ThreadSafeGeneralStore, Flag);
    test_recall_by_name!(ThreadSafeGeneralStore, Role);
    test_recall_by_company!(ThreadSafeGeneralStore, Flag);
    test_recall_by_company!(ThreadSafeGeneralStore, Role);

    // ---- The following tests are more to show how the API of ThreadSafeGeneralStore ----

    #[tokio::test]
    async fn test_base_store() {
        let company = Company::new("name");
        let flag = Flag::new_green(company.id, "good".to_string());
        let role = Role::new(company.id, "role".to_string(), Timestamp::now());

        let mut all_store = ThreadSafeGeneralStore::new(
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
        );

        all_store.store(company.clone()).await.unwrap();
        all_store.store(flag.clone()).await.unwrap();
        all_store.store(role.clone()).await.unwrap();

        let recalled_company: Company = all_store.recall_by_id(&company).await.unwrap();
        let recalled_flag: Flag = all_store.recall_by_id(&flag).await.unwrap();
        let recalled_role: Role = all_store.recall_by_id(&role).await.unwrap();

        assert_eq!(recalled_company, company);
        assert_eq!(recalled_flag, flag);
        assert_eq!(recalled_role, role);
    }

    #[tokio::test]
    async fn test_recall_by_name() {
        let company = Company::new("name");

        let mut all_store = ThreadSafeGeneralStore::new(
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
        );

        all_store.store(company.clone()).await.unwrap();

        let recalled_companies: Vec<Company> = all_store.recall_by_name(&company).await.unwrap();

        assert!(recalled_companies.contains(&company));
    }

    #[tokio::test]
    async fn test_recall_by_company() {
        let company = Company::new("name");
        let flag = Flag::new_green(company.id, "good".to_string());
        let role = Role::new(company.id, "role".to_string(), Timestamp::now());

        let mut all_store = ThreadSafeGeneralStore::new(
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
        );

        all_store.store(flag.clone()).await.unwrap();
        all_store.store(role.clone()).await.unwrap();

        let recalled_flags: Vec<Flag> = all_store.recall_by_company(&company).await.unwrap();
        let recalled_roles: Vec<Role> = all_store.recall_by_company(&company).await.unwrap();

        assert!(recalled_flags.contains(&flag));
        assert!(recalled_roles.contains(&role));
    }
}

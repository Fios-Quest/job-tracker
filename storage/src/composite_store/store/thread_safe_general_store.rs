use crate::composite_store::{GeneralStore, HasFutureStoreFor};
use crate::storable::{Company, Flag, Role};
use crate::storage::{CompanyStore, FlagStore, RoleStore};
use crate::Sealed;
use std::sync::Arc;
use tokio::sync::{MappedMutexGuard, Mutex, MutexGuard};

pub struct ThreadSafeGeneralStore<C, F, R>
where
    C: CompanyStore,
    F: FlagStore,
    R: RoleStore,
{
    general_store: Arc<Mutex<GeneralStore<C, F, R>>>,
}

impl<C, F, R> ThreadSafeGeneralStore<C, F, R>
where
    C: CompanyStore,
    F: FlagStore,
    R: RoleStore,
{
    pub fn new(company_store: C, flag_store: F, role_store: R) -> Self {
        Self {
            general_store: Arc::new(Mutex::new(GeneralStore::new(
                company_store,
                flag_store,
                role_store,
            ))),
        }
    }

    pub async fn company_store(&self) -> MappedMutexGuard<C> {
        MutexGuard::map(self.general_store.lock().await, |lock| {
            lock.company_store_mut()
        })
    }

    pub async fn flag_store(&self) -> MappedMutexGuard<F> {
        MutexGuard::map(self.general_store.lock().await, |lock| {
            lock.flag_store_mut()
        })
    }

    pub async fn role_store(&self) -> MappedMutexGuard<R> {
        MutexGuard::map(self.general_store.lock().await, |lock| {
            lock.role_store_mut()
        })
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

    async fn get_store<'a>(&'a self) -> MappedMutexGuard<'a, Self::Storage>
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

    async fn get_store<'a>(&'a self) -> MappedMutexGuard<'a, Self::Storage>
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

    async fn get_store<'a>(&'a self) -> MappedMutexGuard<'a, Self::Storage>
    where
        Self::Storage: 'a,
    {
        self.role_store().await
    }
}

#[cfg(test)]
mod tests {
    use crate::composite_store::ThreadSafeGeneralStore;
    use crate::storable::{Company, Flag, Role};
    use crate::storage::{BaseStore, RecallByCompany, RecallById, RecallByName, StubStore};
    use crate::Timestamp;

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
        let flag = Flag::new_green(company.id, "good".to_string());
        let role = Role::new(company.id, "role".to_string(), Timestamp::now());

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

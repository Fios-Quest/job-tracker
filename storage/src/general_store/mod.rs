use crate::storable::{Company, Flag, HasId, Role};
use crate::storage::BaseStore;
use crate::storage::{CompanyStore, FlagStore, RoleStore};
use crate::Sealed;

pub mod stub_general_store;

trait HasStoreFor<O, S>: Sealed {
    fn get_mut_store(&mut self) -> &mut S;

    fn get_store(&self) -> &S;
}

pub struct GeneralStore<C, F, R>
where
    C: CompanyStore,
    F: FlagStore,
    R: RoleStore,
{
    company_store: C,
    flag_store: F,
    role_store: R,
}

impl<C, F, R> Sealed for GeneralStore<C, F, R>
where
    C: CompanyStore,
    F: FlagStore,
    R: RoleStore,
{
}

impl<C, F, R> HasStoreFor<Company, C> for GeneralStore<C, F, R>
where
    C: CompanyStore,
    F: FlagStore,
    R: RoleStore,
{
    fn get_mut_store(&mut self) -> &mut C {
        &mut self.company_store
    }

    fn get_store(&self) -> &C {
        &self.company_store
    }
}

impl<C, F, R> HasStoreFor<Flag, F> for GeneralStore<C, F, R>
where
    C: CompanyStore,
    F: FlagStore,
    R: RoleStore,
{
    fn get_mut_store(&mut self) -> &mut F {
        &mut self.flag_store
    }

    fn get_store(&self) -> &F {
        &self.flag_store
    }
}

impl<C, F, R> HasStoreFor<Role, R> for GeneralStore<C, F, R>
where
    C: CompanyStore,
    F: FlagStore,
    R: RoleStore,
{
    fn get_mut_store(&mut self) -> &mut R {
        &mut self.role_store
    }

    fn get_store(&self) -> &R {
        &self.role_store
    }
}

impl<T, O, S> BaseStore<O> for T
where
    O: HasId + Clone,
    T: HasStoreFor<O, S>,
    S: BaseStore<O>,
{
    async fn store(&mut self, storable: O) -> anyhow::Result<()> {
        self.get_mut_store().store(storable).await
    }

    async fn recall_by_id<I: HasId>(&self, id: &I) -> anyhow::Result<O> {
        self.get_store().recall_by_id(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::StubStore;
    use crate::Timestamp;

    #[tokio::test]
    async fn test_store_everything() {
        let company = Company::new("name");
        let flag = Flag::new_green(company.id, "good".to_string());
        let role = Role::new(company.id, "role".to_string(), Timestamp::now());

        let mut all_store = GeneralStore {
            company_store: StubStore::default(),
            flag_store: StubStore::default(),
            role_store: StubStore::default(),
        };

        all_store.store(company.clone()).await.unwrap();
        all_store.store(flag.clone()).await.unwrap();
        all_store.store(role.clone()).await.unwrap();

        let recalled_company = all_store.recall_by_id(&company).await.unwrap();
        let recalled_flag = all_store.recall_by_id(&flag).await.unwrap();
        let recalled_role = all_store.recall_by_id(&role).await.unwrap();

        assert_eq!(recalled_company, company);
        assert_eq!(recalled_flag, flag);
        assert_eq!(recalled_role, role);
    }
}

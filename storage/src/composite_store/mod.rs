use crate::storable::{Company, Flag, HasCompany, HasId, HasName, Role};
use crate::storage::{BaseStore, RecallByCompany, RecallByName};
use crate::storage::{CompanyStore, FlagStore, RoleStore};
use crate::Sealed;

mod general_store;
pub use general_store::*;

mod thread_safe_general_store;
pub use thread_safe_general_store::*;

trait HasStoreFor<O>: Sealed {
    type Storage;
    type MutStorage;

    async fn get_mut_store<'a>(&'a mut self) -> &mut Self::MutStorage
    where
        <Self as HasStoreFor<O>>::MutStorage: 'a;

    async fn get_store<'a>(&'a self) -> &Self::Storage
    where
        <Self as HasStoreFor<O>>::Storage: 'a;
}

impl<T, O> BaseStore<O> for T
where
    T: HasStoreFor<O>,
    T::Storage: BaseStore<O>,
    T::MutStorage: BaseStore<O>,
    O: HasId + Clone,
{
    async fn store(&mut self, storable: O) -> anyhow::Result<()> {
        self.get_mut_store().await.store(storable).await
    }

    async fn recall_by_id<I: HasId>(&self, id: &I) -> anyhow::Result<O> {
        self.get_store().await.recall_by_id(id).await
    }
}

impl<T, O> RecallByName<O> for T
where
    T: HasStoreFor<O>,
    T::Storage: RecallByName<O>,
    T::MutStorage: RecallByName<O>,
    O: HasName + Clone,
{
    async fn recall_by_name<N: HasName>(&self, name: N) -> anyhow::Result<Vec<O>> {
        self.get_store().await.recall_by_name(name).await
    }
}

impl<T, O> RecallByCompany<O> for T
where
    T: HasStoreFor<O>,
    T::Storage: RecallByCompany<O>,
    T::MutStorage: RecallByCompany<O>,
    O: HasCompany + Clone,
{
    async fn recall_by_company<I: HasId>(&self, company_id: &I) -> anyhow::Result<Vec<O>> {
        self.get_store().await.recall_by_company(company_id).await
    }
}

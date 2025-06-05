use crate::storable::{HasCompany, HasId, HasName};
use crate::storage::{BaseStore, RecallByCompany, RecallById, RecallByName};
use crate::Sealed;

mod general_store;
pub use general_store::*;

// mod thread_safe_general_store;
// pub use thread_safe_general_store::*;

trait HasStoreFor<O>: Sealed {
    type Storage;

    fn get_store(&self) -> &Self::Storage;
}

trait HasMutStoreFor<O>: Sealed {
    type Storage;

    fn get_mut_store(&mut self) -> &mut Self::Storage;
}

impl<T, O> BaseStore<O> for T
where
    T: HasMutStoreFor<O>,
    T::Storage: BaseStore<O>,
    O: HasId + Clone,
{
    async fn store(&mut self, storable: O) -> anyhow::Result<()> {
        self.get_mut_store().store(storable).await
    }
}

impl<T, O> RecallById<O> for T
where
    T: HasStoreFor<O>,
    T::Storage: RecallById<O>,
    O: HasId + Clone,
{
    async fn recall_by_id<I: HasId>(&self, id: &I) -> anyhow::Result<O> {
        self.get_store().recall_by_id(id).await
    }
}

impl<T, O> RecallByName<O> for T
where
    T: HasStoreFor<O>,
    T::Storage: RecallByName<O>,
    O: HasName + Clone,
{
    async fn recall_by_name<N: HasName>(&self, name: N) -> anyhow::Result<Vec<O>> {
        self.get_store().recall_by_name(name).await
    }
}

impl<T, O> RecallByCompany<O> for T
where
    T: HasStoreFor<O>,
    T::Storage: RecallByCompany<O>,
    O: HasCompany + Clone,
{
    async fn recall_by_company<I: HasId>(&self, company_id: &I) -> anyhow::Result<Vec<O>> {
        self.get_store().recall_by_company(company_id).await
    }
}

use crate::storable::{HasCompany, HasId, HasName};
use crate::storage::{BaseStore, RecallByCompany, RecallById, RecallByName};
use crate::Sealed;
use tokio::sync::MappedMutexGuard;

pub trait HasFutureStoreFor<O>: Sealed {
    type Storage;

    async fn get_store<'a>(&'a self) -> MappedMutexGuard<Self::Storage>
    where
        <Self as HasFutureStoreFor<O>>::Storage: 'a;
}

impl<T, O> BaseStore<O> for T
where
    T: HasFutureStoreFor<O>,
    T::Storage: BaseStore<O>,
    O: HasId + Clone,
{
    async fn store(&mut self, storable: O) -> anyhow::Result<()> {
        self.get_store().await.store(storable).await
    }
}

impl<T, O> RecallById<O> for T
where
    T: HasFutureStoreFor<O>,
    T::Storage: RecallById<O>,
    O: HasId + Clone,
{
    async fn recall_by_id<I: HasId>(&self, id: &I) -> anyhow::Result<O> {
        self.get_store().await.recall_by_id(id).await
    }
}

impl<T, O> RecallByName<O> for T
where
    T: HasFutureStoreFor<O>,
    T::Storage: RecallByName<O>,
    O: HasName + Clone,
{
    async fn recall_by_name<N: HasName>(&self, name: N) -> anyhow::Result<Vec<O>> {
        self.get_store().await.recall_by_name(name).await
    }
}

impl<T, O> RecallByCompany<O> for T
where
    T: HasFutureStoreFor<O>,
    T::Storage: RecallByCompany<O>,
    O: HasCompany + Clone,
{
    async fn recall_by_company<I: HasId>(&self, company_id: &I) -> anyhow::Result<Vec<O>> {
        self.get_store().await.recall_by_company(company_id).await
    }
}

use crate::storable::property::has_id::HasId;
use anyhow::Result;

pub trait BaseStore<T>
where
    T: HasId + Clone,
{
    async fn store(&mut self, storable: T) -> Result<()>;

    async fn recall_by_id<I: HasId>(&self, id: I) -> Result<T>;
}

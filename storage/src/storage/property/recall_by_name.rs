use crate::storable::HasName;
use anyhow::Result;

pub trait RecallByName<T>
where
    T: HasName + Clone,
{
    async fn recall_by_name<N: HasName>(&self, name: N) -> Result<Vec<T>>;
}

use super::{Flag, FlagStore};
use crate::error::StorageError;
use crate::store::StubStore;
use async_trait::async_trait;
use uuid::Uuid;

pub type StubFlagStore = StubStore<Flag>;

#[async_trait]
impl FlagStore for StubFlagStore {
    async fn get_for_company(&self, id: Uuid) -> Result<Vec<Flag>, StorageError> {
        Ok(self
            .store
            .iter()
            .filter(|f| f.company_id == id)
            .cloned()
            .collect())
    }
}

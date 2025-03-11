use crate::flag::{Flag, FlagStore};
use crate::{StorageError, StubStore};
use uuid::Uuid;

pub type StubFlagStore = StubStore<Flag>;

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

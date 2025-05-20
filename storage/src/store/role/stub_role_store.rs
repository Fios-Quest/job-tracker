use super::{Role, RoleStore};
use crate::error::StorageError;
use crate::store::StubStore;
use async_trait::async_trait;
use uuid::Uuid;

pub type StubRoleStore = StubStore<Role>;

#[async_trait]
impl RoleStore for StubRoleStore {
    async fn get_for_company(&self, id: Uuid) -> Result<Vec<Role>, StorageError> {
        Ok(self
            .store
            .iter()
            .filter(|c| c.company_id == id)
            .cloned()
            .collect())
    }
}

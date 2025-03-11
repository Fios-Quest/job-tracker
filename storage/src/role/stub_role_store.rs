use crate::role::{Role, RoleStore};
use crate::{StorageError, StubStore};
use uuid::Uuid;

pub type StubRoleStore = StubStore<Role>;

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

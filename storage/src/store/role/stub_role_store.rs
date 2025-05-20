use super::{Role, RoleStore};
use crate::store::StubStore;
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

pub type StubRoleStore = StubStore<Role>;

#[async_trait]
impl RoleStore for StubRoleStore {
    async fn get_for_company(&self, id: Uuid) -> Result<Vec<Role>> {
        Ok(self
            .store
            .iter()
            .filter(|c| c.company_id == id)
            .cloned()
            .collect())
    }
}

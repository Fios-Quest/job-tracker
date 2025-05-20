use super::{Role, RoleStore};
use crate::error::StorageError;
use crate::store::json::{JsonStore, JsonStoreConstructor};
use crate::StubRoleStore;
use async_trait::async_trait;
use uuid::Uuid;

pub type JsonRoleStore = JsonStore<Role>;

impl JsonStoreConstructor<Role> for JsonRoleStore {
    fn create_stub_store() -> StubRoleStore {
        StubRoleStore::new()
    }
}

#[async_trait]
impl RoleStore for JsonRoleStore {
    async fn get_for_company(&self, id: Uuid) -> Result<Vec<Role>, StorageError> {
        self.internal_store.get_for_company(id).await
    }
}

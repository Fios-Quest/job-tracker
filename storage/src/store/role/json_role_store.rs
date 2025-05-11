use super::{Role, RoleStore};
use crate::store::json::{JsonStore, JsonStoreConstructor};
use crate::store::StorageError;
use crate::StubRoleStore;
use async_trait::async_trait;
use uuid::Uuid;

pub type JsonRoleStore = JsonStore<Role, StubRoleStore>;

impl JsonStoreConstructor<Role, StubRoleStore> for JsonRoleStore {
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

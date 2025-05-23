use crate::store::json::{JsonStore, JsonStoreConstructor};
use crate::{Flag, FlagStore, StubFlagStore};
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

pub type JsonFlagStore = JsonStore<Flag>;

impl JsonStoreConstructor<Flag> for JsonFlagStore {
    fn create_stub_store() -> StubFlagStore {
        StubFlagStore::new()
    }
}

#[async_trait]
impl FlagStore for JsonFlagStore {
    async fn get_for_company(&self, id: Uuid) -> Result<Vec<Flag>> {
        Ok(self.internal_store.get_for_company(id).await?)
    }
}

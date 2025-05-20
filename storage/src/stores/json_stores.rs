use crate::{JsonCompanyStore, JsonFlagStore, JsonRoleStore, Stores};
use anyhow::Result;
use std::path::PathBuf;

pub struct JsonStores {
    company_store: JsonCompanyStore,
    role_store: JsonRoleStore,
    flag_store: JsonFlagStore,
}

impl JsonStores {
    pub async fn new(base_path: PathBuf) -> Result<Self> {
        Ok(JsonStores {
            company_store: JsonCompanyStore::new(base_path.join("company")).await?,
            role_store: JsonRoleStore::new(base_path.join("role")).await?,
            flag_store: JsonFlagStore::new(base_path.join("flag")).await?,
        })
    }
}

impl Stores<JsonCompanyStore, JsonRoleStore, JsonFlagStore> for JsonStores {
    fn company_store(&mut self) -> &mut JsonCompanyStore {
        &mut self.company_store
    }

    fn role_store(&mut self) -> &mut JsonRoleStore {
        &mut self.role_store
    }

    fn flag_store(&mut self) -> &mut JsonFlagStore {
        &mut self.flag_store
    }
}

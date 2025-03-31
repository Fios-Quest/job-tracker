use crate::store::StorageError;
use crate::{RocksCompanyStore, RocksFlagStore, RocksRoleStore, Stores};
use std::path::PathBuf;
use surrealdb::engine::local::RocksDb;
use surrealdb::Surreal;

pub struct RocksStores {
    company_store: RocksCompanyStore,
    role_store: RocksRoleStore,
    flag_store: RocksFlagStore,
}

impl RocksStores {
    pub async fn new(path_to_db: PathBuf) -> Result<Self, StorageError> {
        let db = Surreal::new::<RocksDb>(path_to_db).await?;
        db.use_ns("test").use_db("test").await?;

        let company_store = RocksCompanyStore::new(db.clone());
        let role_store = RocksRoleStore::new(db.clone());
        let flag_store = RocksFlagStore::new(db.clone());

        Ok(Self {
            company_store,
            role_store,
            flag_store,
        })
    }
}

impl Stores<RocksCompanyStore, RocksRoleStore, RocksFlagStore> for RocksStores {
    fn company_store(&mut self) -> &mut RocksCompanyStore {
        &mut self.company_store
    }

    fn role_store(&mut self) -> &mut RocksRoleStore {
        &mut self.role_store
    }

    fn flag_store(&mut self) -> &mut RocksFlagStore {
        &mut self.flag_store
    }
}

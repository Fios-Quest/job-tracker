use crate::store::{RocksStore, StorageError};
use crate::utils::*;
use std::marker::PhantomData;
use std::path::Path;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;

impl From<surrealdb::Error> for StorageError {
    fn from(e: surrealdb::Error) -> Self {
        StorageError::SurrealError(e.to_string())
    }
}

impl<T> RocksStore<T>
where
    T: GetName + GetId + GetDeleted + SetDeleted + Clone + Send + Sync,
{
    pub fn new(db: Surreal<Db>) -> Self {
        let phantom_data = PhantomData;
        Self { phantom_data, db }
    }

    pub async fn new_from_path(path: &Path) -> Result<Self, StorageError> {
        let db = Surreal::new::<RocksDb>(path).await?;
        db.use_ns("test").use_db("test").await?;
        Ok(Self::new(db))
    }

    #[cfg(test)]
    pub async fn new_tmp() -> Result<Self, StorageError> {
        let path = tempdir::TempDir::new("company_test").unwrap();
        Self::new_from_path(path.as_ref()).await
    }
}

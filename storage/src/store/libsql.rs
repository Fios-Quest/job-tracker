use crate::{GetDeleted, GetId, GetName, LibSqlStore, SetDeleted, StorageError};
use async_trait::async_trait;
use libsql::{Builder, Connection, OpenFlags};
use serde::de;
use std::marker::PhantomData;
use std::path::Path;

#[async_trait]
pub trait HasLibSqlTable {
    async fn migrate(db: &Connection) -> Result<(), StorageError>;
}

impl From<libsql::Error> for StorageError {
    fn from(e: libsql::Error) -> Self {
        StorageError::LibSqlError(e.to_string())
    }
}

impl From<de::value::Error> for StorageError {
    fn from(e: de::value::Error) -> Self {
        StorageError::DeserializationError(e)
    }
}

impl<T> LibSqlStore<T>
where
    T: GetName + GetId + GetDeleted + SetDeleted + Clone + Send + Sync + HasLibSqlTable,
{
    pub fn new(conn: Connection) -> Self {
        let phantom_data = PhantomData;
        Self { phantom_data, conn }
    }

    pub async fn new_from_path(path: &Path) -> Result<Self, StorageError> {
        let conn = Builder::new_local(path)
            .flags(OpenFlags::SQLITE_OPEN_CREATE)
            .build()
            .await?
            .connect()?;
        T::migrate(&conn).await?;
        Ok(Self::new(conn))
    }

    #[cfg(test)]
    pub async fn new_tmp() -> Result<Self, StorageError> {
        Self::new_from_path(Path::new(":memory:")).await
    }
}

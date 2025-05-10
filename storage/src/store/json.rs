use crate::{GetDeleted, GetId, GetName, SetDeleted, StorageError, Store, StubStore, Timestamp};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use std::ffi::OsStr;
use std::marker::PhantomData;
use std::path::PathBuf;
use tokio::fs::{create_dir_all, read, read_dir};
use uuid::Uuid;

impl From<tokio::io::Error> for StorageError {
    fn from(err: tokio::io::Error) -> Self {
        Self::TokioIoError(err.to_string())
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJsonError(err.to_string())
    }
}

pub struct JsonStore<T, S>
where
    T: GetName
        + GetId
        + GetDeleted
        + SetDeleted
        + Clone
        + Send
        + Sync
        + Serialize
        + DeserializeOwned,
    S: Store<T> + Send + Sync,
{
    base_path: PathBuf,
    internal_store: StubStore<T>,
    phantom_item: PhantomData<T>,
    phantom_store: PhantomData<S>,
}

impl<T, S> JsonStore<T, S>
where
    T: GetName
        + GetId
        + GetDeleted
        + SetDeleted
        + Clone
        + Send
        + Sync
        + Serialize
        + DeserializeOwned,
    S: Store<T> + Send + Sync,
{
    pub async fn new(
        base_path: PathBuf,
        mut internal_store: StubStore<T>,
    ) -> Result<Self, StorageError> {
        create_dir_all(&base_path).await?;
        let mut dir = read_dir(&base_path).await?;

        while let Some(entry) = dir.next_entry().await? {
            if entry.path().is_file() && entry.path().extension() == Some(OsStr::new("json")) {
                let file_date = read(&entry.path()).await?;
                let item: T = serde_json::from_slice(&file_date)?;
                internal_store.create(item).await?;
            }
        }

        Ok(Self {
            base_path,
            internal_store,
            phantom_item: PhantomData,
            phantom_store: PhantomData,
        })
    }

    #[cfg(test)]
    pub async fn new_tmp(internal_store: StubStore<T>) -> Result<Self, StorageError> {
        let mut base_path = std::env::temp_dir();
        base_path.push(Uuid::new_v4().to_string());
        Self::new(base_path, internal_store).await
    }

    pub fn create_filename(&self, data: &T) -> PathBuf {
        let mut buf = self.base_path.clone();
        buf.push(format!("{}.json", data.get_id()));
        buf
    }

    async fn write_file(&self, data: &T) -> Result<(), StorageError> {
        let path = self.create_filename(data);
        tokio::fs::write(path, json!(data).to_string().as_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
impl<T, S> Store<T> for JsonStore<T, S>
where
    T: GetName
        + GetId
        + GetDeleted
        + SetDeleted
        + Clone
        + Send
        + Sync
        + Serialize
        + DeserializeOwned,
    S: Store<T> + Send + Sync,
{
    async fn get_by_id(&self, id: Uuid) -> Result<T, StorageError> {
        self.internal_store.get_by_id(id).await
    }

    async fn get_by_name(&self, name: &str) -> Result<T, StorageError> {
        self.internal_store.get_by_name(name).await
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<T>, StorageError> {
        self.internal_store.find_by_name(name).await
    }

    async fn create(&mut self, item: T) -> Result<(), StorageError> {
        self.write_file(&item).await?;
        self.internal_store.create(item).await?;
        Ok(())
    }

    async fn update(&mut self, item: T) -> Result<(), StorageError> {
        self.write_file(&item).await?;
        self.internal_store.update(item).await?;
        Ok(())
    }

    async fn delete_by_id(
        &mut self,
        id: Uuid,
        date_deleted: Timestamp,
    ) -> Result<(), StorageError> {
        let mut item = self.internal_store.get_by_id(id).await?;
        item.set_deleted(date_deleted);
        self.write_file(&item).await?;
        self.internal_store.delete_by_id(id, date_deleted).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Company, StubCompanyStore};

    #[tokio::test]
    async fn test_create_filename() {
        let store = JsonStore::<Company, StubCompanyStore>::new_tmp(StubCompanyStore::new())
            .await
            .expect("Could not create store");

        let company = Company::new("Test Company".to_string());
        let filename = store.create_filename(&company);

        let iter = filename.iter();
        assert!(iter.last().unwrap().to_str().unwrap().ends_with(".json"));
    }
}

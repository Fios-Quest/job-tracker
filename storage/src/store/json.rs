use crate::{GetDeleted, GetId, GetName, SetDeleted, Store, StubStore, Timestamp};
use anyhow::Result;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use std::ffi::OsStr;
use std::marker::PhantomData;
use std::path::PathBuf;
use tokio::fs::{create_dir_all, read, read_dir};
use uuid::Uuid;

#[async_trait]
pub trait JsonStoreConstructor<T> {
    fn create_stub_store() -> StubStore<T>;
}

pub struct JsonStore<T>
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
{
    base_path: PathBuf,
    pub(crate) internal_store: StubStore<T>,
    phantom_item: PhantomData<T>,
}

impl<T> JsonStore<T>
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
    Self: JsonStoreConstructor<T>,
{
    pub async fn new(base_path: PathBuf) -> Result<Self> {
        let mut internal_store = Self::create_stub_store();
        create_dir_all(&base_path).await?;
        let mut dir = read_dir(&base_path).await?;

        while let Some(entry) = dir.next_entry().await? {
            if entry.path().is_file() && entry.path().extension() == Some(OsStr::new("json")) {
                let file_date = read(&entry.path()).await?;
                // ToDo: Could create a From<AsRef<T>> for some borrowed form of T to avoid double
                //       alloc caused by StubStore in this one case
                let item: T = serde_json::from_slice(&file_date)?;
                internal_store.create(&item).await?;
            }
        }

        Ok(Self {
            base_path,
            internal_store,
            phantom_item: PhantomData,
        })
    }

    #[cfg(test)]
    pub async fn new_tmp() -> Result<Self> {
        let base_path = tempdir::TempDir::new("company_test")?;
        Self::new(base_path.into_path()).await
    }

    pub fn create_filename(&self, data: &T) -> PathBuf {
        let mut buf = self.base_path.clone();
        buf.push(data.get_id().to_string());
        buf.set_extension("json");
        buf
    }

    async fn write_file(&self, data: &T) -> Result<()> {
        let path = self.create_filename(data);
        tokio::fs::write(path, json!(data).to_string().as_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
impl<T> Store<T> for JsonStore<T>
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
    Self: JsonStoreConstructor<T>,
{
    async fn get_by_id(&self, id: Uuid) -> Result<T> {
        Ok(self.internal_store.get_by_id(id).await?)
    }

    async fn get_by_name(&self, name: &str) -> Result<T> {
        self.internal_store.get_by_name(name).await
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<T>> {
        self.internal_store.find_by_name(name).await
    }

    async fn create(&mut self, item: &T) -> Result<()> {
        self.internal_store.create(item).await?;
        self.write_file(item).await?;
        Ok(())
    }

    async fn update(&mut self, item: &T) -> Result<()> {
        self.internal_store.update(item).await?;
        self.write_file(item).await?;
        Ok(())
    }

    async fn delete_by_id(&mut self, id: Uuid, date_deleted: Timestamp) -> Result<()> {
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
    use crate::Company;

    #[tokio::test]
    async fn test_create_filename() {
        let store = JsonStore::<Company>::new_tmp()
            .await
            .expect("Could not create store");

        let company = Company::new("Test Company".to_string());
        let filename = store.create_filename(&company);

        let iter = filename.iter();
        assert!(iter.last().unwrap().to_str().unwrap().ends_with(".json"));
    }
}

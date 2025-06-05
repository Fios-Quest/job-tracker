use crate::storable::{Company, Flag, HasCompany, HasId, HasName, Role};
use crate::storage::{
    BaseStore, CompanyStore, FlagStore, RecallByCompany, RecallById, RecallByName, RoleStore,
    StubStore,
};
use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use std::ffi::OsStr;
use std::path::PathBuf;
use tokio::fs::{create_dir_all, read, read_dir};

pub struct JsonStore<O> {
    base_path: PathBuf,
    pub(crate) internal_store: StubStore<O>,
}

impl<O> JsonStore<O>
where
    O: Clone + HasId + Serialize + DeserializeOwned,
{
    pub async fn new(base_path: PathBuf) -> Result<Self> {
        let mut internal_store = StubStore::default();
        create_dir_all(&base_path).await?;
        let mut dir = read_dir(&base_path).await?;

        while let Some(entry) = dir.next_entry().await? {
            if entry.path().is_file() && entry.path().extension() == Some(OsStr::new("json")) {
                let file_date = read(&entry.path()).await?;
                let item: O = serde_json::from_slice(&file_date)?;
                internal_store.store(item).await?;
            }
        }

        Ok(Self {
            base_path,
            internal_store,
        })
    }

    pub fn create_filename(&self, data: &O) -> PathBuf {
        let mut buf = self.base_path.clone();
        buf.push(data.get_id().to_string());
        buf.set_extension("json");
        buf
    }

    async fn write_file(&self, data: &O) -> Result<()> {
        let path = self.create_filename(data);
        tokio::fs::write(path, json!(data).to_string().as_bytes()).await?;
        Ok(())
    }
}

impl<O> BaseStore<O> for JsonStore<O>
where
    O: HasId + Clone + Serialize + DeserializeOwned,
{
    async fn store(&mut self, storable: O) -> anyhow::Result<()> {
        self.write_file(&storable).await?;
        self.internal_store.store(storable).await?;
        Ok(())
    }
}

impl<O> RecallById<O> for JsonStore<O>
where
    O: HasId + Clone + Serialize + DeserializeOwned,
{
    async fn recall_by_id<I: HasId>(&self, id: &I) -> anyhow::Result<O> {
        self.internal_store.recall_by_id(id).await
    }
}

impl<T> RecallByName<T> for JsonStore<T>
where
    T: HasName + Clone + Serialize + DeserializeOwned,
{
    async fn recall_by_name<N: HasName>(&self, name: N) -> anyhow::Result<Vec<T>> {
        self.internal_store.recall_by_name(name).await
    }
}

impl<T> RecallByCompany<T> for JsonStore<T>
where
    T: HasCompany + Clone + Serialize + DeserializeOwned,
{
    async fn recall_by_company<C: HasId>(&self, company: &C) -> anyhow::Result<Vec<T>> {
        self.internal_store.recall_by_company(company).await
    }
}

impl CompanyStore for JsonStore<Company> {}
impl RoleStore for JsonStore<Role> {}
impl FlagStore for JsonStore<Flag> {}

#[cfg(test)]
mod test_helper {
    use super::*;
    use crate::test_helper::TestHelper;

    #[cfg(test)]
    impl<O> TestHelper for JsonStore<O>
    where
        O: HasId + Clone + Serialize + DeserializeOwned,
    {
        #[cfg(test)]
        async fn new_test() -> Result<Self> {
            let base_path = tempdir::TempDir::new("json_store_test")?;
            Self::new(base_path.into_path()).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::{
        base_store::test_helper::test_base_store,
        recall_by_company::test_helper::test_recall_by_company,
        recall_by_name::test_helper::test_recall_by_name,
    };
    use crate::test_helper::*;
    use paste::paste;

    test_base_store!(JsonStore, Company);
    test_base_store!(JsonStore, Flag);
    test_base_store!(JsonStore, Role);
    test_recall_by_name!(JsonStore, Company);
    test_recall_by_name!(JsonStore, Flag);
    test_recall_by_name!(JsonStore, Role);
    test_recall_by_company!(JsonStore, Flag);
    test_recall_by_company!(JsonStore, Role);

    #[tokio::test]
    async fn test_load_from_file() {
        let base_path = tempdir::TempDir::new("json_store_test")
            .unwrap()
            .into_path();

        let company = Company::new("company");

        // Scoped store to drop it
        {
            let mut initial_store = JsonStore::new(base_path.clone()).await.unwrap();
            initial_store.store(company.clone()).await.unwrap();
        }
        // Second scoped store
        let recalled_company = {
            let loaded_store = JsonStore::<Company>::new(base_path).await.unwrap();
            loaded_store.recall_by_id(&company.get_id()).await.unwrap()
        };

        assert_eq!(recalled_company, company);
    }
}

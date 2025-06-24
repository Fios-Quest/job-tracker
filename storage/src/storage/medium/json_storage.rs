use crate::prelude::Interview;
use crate::storable::{
    Company, Flag, HasCompany, HasDeleted, HasId, HasName, HasRole, Question, Role, Value,
};
use crate::storage::{
    BaseStore, CompanyStore, FlagStore, InterviewStore, QuestionStore, RecallByCompany, RecallById,
    RecallByName, RecallByRole, RoleStore, StubStore, ValueStore,
};
use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use std::ffi::OsStr;
use std::path::PathBuf;
use tokio::fs::{create_dir_all, read, read_dir};

#[derive(Clone)]
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

pub trait ScopedJsonStoreFor
where
    Self: Sized,
{
    async fn new_scoped(base_path: PathBuf) -> Result<Self>;
}

impl ScopedJsonStoreFor for JsonStore<Company> {
    async fn new_scoped(mut base_path: PathBuf) -> Result<Self> {
        base_path.push("company");
        Self::new(base_path).await
    }
}

impl ScopedJsonStoreFor for JsonStore<Flag> {
    async fn new_scoped(mut base_path: PathBuf) -> Result<Self> {
        base_path.push("flag");
        Self::new(base_path).await
    }
}

impl ScopedJsonStoreFor for JsonStore<Value> {
    async fn new_scoped(mut base_path: PathBuf) -> Result<Self> {
        base_path.push("value");
        Self::new(base_path).await
    }
}

impl ScopedJsonStoreFor for JsonStore<Role> {
    async fn new_scoped(mut base_path: PathBuf) -> Result<Self> {
        base_path.push("role");
        Self::new(base_path).await
    }
}

impl ScopedJsonStoreFor for JsonStore<Interview> {
    async fn new_scoped(mut base_path: PathBuf) -> Result<Self> {
        base_path.push("interview");
        Self::new(base_path).await
    }
}

impl ScopedJsonStoreFor for JsonStore<Question> {
    async fn new_scoped(mut base_path: PathBuf) -> Result<Self> {
        base_path.push("question");
        Self::new(base_path).await
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
    O: HasId + HasDeleted + Clone + Serialize + DeserializeOwned,
{
    async fn recall_by_id<I: HasId>(&self, id: I) -> anyhow::Result<O> {
        self.internal_store.recall_by_id(id).await
    }
}

impl<T> RecallByName<T> for JsonStore<T>
where
    T: HasName + HasDeleted + Clone + Serialize + DeserializeOwned,
{
    async fn recall_by_name<N: AsRef<str>>(&self, name: N) -> anyhow::Result<Vec<T>> {
        self.internal_store.recall_by_name(name).await
    }
}

impl<T> RecallByCompany<T> for JsonStore<T>
where
    T: HasCompany + HasDeleted + Clone + Serialize + DeserializeOwned,
{
    async fn recall_by_company<C: HasId>(&self, company: C) -> anyhow::Result<Vec<T>> {
        self.internal_store.recall_by_company(company).await
    }
}

impl<T> RecallByRole<T> for JsonStore<T>
where
    T: HasRole + HasDeleted + Clone + Serialize + DeserializeOwned,
{
    async fn recall_by_role<R: HasId>(&self, role: R) -> anyhow::Result<Vec<T>> {
        self.internal_store.recall_by_role(role).await
    }
}

impl CompanyStore for JsonStore<Company> {}
impl RoleStore for JsonStore<Role> {}
impl FlagStore for JsonStore<Flag> {}
impl QuestionStore for JsonStore<Question> {}
impl InterviewStore for JsonStore<Interview> {}

impl ValueStore for JsonStore<Value> {}

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
        recall_by_company::test_helper::test_recall_by_company,
        recall_by_id::test_helper::test_recall_by_id,
        recall_by_name::test_helper::test_recall_by_name,
        recall_by_role::test_helper::test_recall_by_role,
    };
    use crate::test_helper::*;
    use paste::paste;
    use std::fs::File;
    use std::io::Write;

    test_recall_by_id!(JsonStore, Company);
    test_recall_by_id!(JsonStore, Flag);
    test_recall_by_id!(JsonStore, Role);
    test_recall_by_id!(JsonStore, Value);
    test_recall_by_id!(JsonStore, Question);
    test_recall_by_id!(JsonStore, Interview);
    test_recall_by_name!(JsonStore, Company);
    test_recall_by_name!(JsonStore, Flag);
    test_recall_by_name!(JsonStore, Role);
    test_recall_by_name!(JsonStore, Value);
    test_recall_by_name!(JsonStore, Question);
    test_recall_by_name!(JsonStore, Interview);
    test_recall_by_company!(JsonStore, Flag);
    test_recall_by_company!(JsonStore, Role);
    test_recall_by_company!(JsonStore, Value);
    test_recall_by_role!(JsonStore, Question);
    test_recall_by_role!(JsonStore, Interview);

    #[tokio::test]
    async fn test_load_from_file() {
        let base_path = tempdir::TempDir::new("json_store_test")
            .unwrap()
            .into_path();

        let company = Company::new("company");

        // Store some actual json
        let mut initial_store = JsonStore::new(base_path.clone()).await.unwrap();
        initial_store.store(company.clone()).await.unwrap();
        drop(initial_store);

        // Create a dummy file
        let file_name = base_path.join("test.txt");
        let mut file = File::create(file_name).unwrap();
        file.write_all(b"I am not json!").unwrap();
        drop(file);

        // Read the json back (but not the dummy file)
        let loaded_store = JsonStore::<Company>::new(base_path).await.unwrap();
        let recalled_company = loaded_store.recall_by_id(&company.get_id()).await.unwrap();

        assert_eq!(recalled_company, company);
    }

    #[tokio::test]
    async fn test_company_scoped() {
        let base_path = tempdir::TempDir::new("json_store_test")
            .unwrap()
            .into_path();

        let company_store = JsonStore::<Company>::new_scoped(base_path).await.unwrap();
        assert!(company_store.base_path.ends_with("company"));
    }

    #[tokio::test]
    async fn test_flag_scoped() {
        let base_path = tempdir::TempDir::new("json_store_test")
            .unwrap()
            .into_path();

        let flag_store = JsonStore::<Flag>::new_scoped(base_path).await.unwrap();
        assert!(flag_store.base_path.ends_with("flag"));
    }

    #[tokio::test]
    async fn test_role_scoped() {
        let base_path = tempdir::TempDir::new("json_store_test")
            .unwrap()
            .into_path();

        let role_store = JsonStore::<Role>::new_scoped(base_path).await.unwrap();
        assert!(role_store.base_path.ends_with("role"));
    }
}

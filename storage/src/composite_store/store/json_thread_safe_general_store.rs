use crate::composite_store::ThreadSafeGeneralStore;
use crate::prelude::Value;
use crate::storable::{Company, Flag, Interview, Question, Role};
use crate::storage::{JsonStore, ScopedJsonStoreFor};
use anyhow::Result;
use std::path::PathBuf;
use tokio::join;

pub type JsonThreadSafeGeneralStore = ThreadSafeGeneralStore<
    JsonStore<Company>,
    JsonStore<Flag>,
    JsonStore<Value>,
    JsonStore<Role>,
    JsonStore<Interview>,
    JsonStore<Question>,
>;

impl JsonThreadSafeGeneralStore {
    pub async fn new_json(base_path: PathBuf) -> Result<Self> {
        let (company_store, flag_store, value_store, role_store, interview_store, question_store) = join!(
            JsonStore::<Company>::new_scoped(base_path.clone()),
            JsonStore::<Flag>::new_scoped(base_path.clone()),
            JsonStore::<Value>::new_scoped(base_path.clone()),
            JsonStore::<Role>::new_scoped(base_path.clone()),
            JsonStore::<Interview>::new_scoped(base_path.clone()),
            JsonStore::<Question>::new_scoped(base_path.clone()),
        );

        Ok(Self::new(
            company_store?,
            flag_store?,
            value_store?,
            role_store?,
            interview_store?,
            question_store?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_json() {
        let base_path = tempdir::TempDir::new("json_store_test")
            .unwrap()
            .into_path();

        assert!(JsonThreadSafeGeneralStore::new_json(base_path)
            .await
            .is_ok());
    }
}

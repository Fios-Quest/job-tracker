mod stub_flag_store;

use std::fmt;
pub use stub_flag_store::StubFlagStore;

mod rocks_flag_store;
pub use rocks_flag_store::RocksFlagStore;

mod libsql_flag_store;
pub use libsql_flag_store::LibSqlFlagStore;

use crate::store::{StorageError, Store};
use crate::utils::{GetDeleted, GetId, GetName, SetDeleted};
use crate::Timestamp;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlagColor {
    Green,
    Red,
}

impl FromStr for FlagColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "green" => Ok(Self::Green),
            "red" => Ok(Self::Red),
            _ => Err(format!("Invalid flag_color '{}'", s)),
        }
    }
}

impl fmt::Display for FlagColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlagColor::Green => write!(f, "green"),
            FlagColor::Red => write!(f, "red"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flag {
    pub id: Uuid,
    pub company_id: Uuid,
    pub flag_color: FlagColor,
    pub name: String,
    pub date_deleted: Option<Timestamp>,
}

impl PartialEq for Flag {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.company_id == other.company_id
            && self.flag_color == other.flag_color
            && self.name == other.name
    }
}

impl GetName for Flag {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl GetId for Flag {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl GetDeleted for Flag {
    fn get_deleted(&self) -> Option<Timestamp> {
        self.date_deleted
    }
}

impl SetDeleted for Flag {
    fn set_deleted(&mut self, time: Timestamp) {
        self.date_deleted = Some(time);
    }
}

impl Flag {
    pub fn new_green(company_id: Uuid, name: String) -> Self {
        Flag {
            id: Uuid::new_v4(),
            company_id,
            flag_color: FlagColor::Green,
            name,
            date_deleted: None,
        }
    }

    pub fn new_red(company_id: Uuid, name: String) -> Self {
        Flag {
            id: Uuid::new_v4(),
            company_id,
            flag_color: FlagColor::Red,
            name,
            date_deleted: None,
        }
    }
}

#[async_trait]
pub trait FlagStore<T: Store<Flag> = Self>: Store<Flag> {
    async fn get_for_company(&self, id: Uuid) -> Result<Vec<Flag>, StorageError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::Store;

    // Reusable test functions
    async fn test_get_by_id<C: Store<Flag>>(store: &mut C) {
        let flag = Flag::new_green(Uuid::new_v4(), "Test".to_string());
        assert_eq!(store.create(flag.clone()).await, Ok(()));

        assert_eq!(flag.id, store.get_by_id(flag.id).await.unwrap().id);
    }

    async fn test_get_by_name<C: Store<Flag>>(store: &mut C) {
        let name = "Test";
        let flag = Flag::new_red(Uuid::new_v4(), name.to_string());
        assert_eq!(store.create(flag).await, Ok(()));

        // Test can be found
        assert_eq!(name, store.get_by_name(name).await.unwrap().name);
        // Test no partial match
        assert_eq!(
            Err(StorageError::NotFound),
            store.get_by_name(&name[..1]).await
        );
    }

    async fn test_find_by_name<C: Store<Flag>>(store: &mut C) {
        let name = "Test";
        let t_flag = Flag::new_red(Uuid::new_v4(), name.to_string());
        assert_eq!(store.create(t_flag.clone()).await, Ok(()));

        // Test can be found with exact match
        assert!(!store.find_by_name(name).await.unwrap().is_empty());
        // Test can be found with partial match
        assert!(!store.find_by_name(&name[..1]).await.unwrap().is_empty());

        // It should return all companies when search string is empty
        let a_flag = Flag::new_green(Uuid::new_v4(), "Another flag".to_string());
        let y_flag = Flag::new_green(Uuid::new_v4(), "Yet Another flag".to_string());
        assert_eq!(store.create(a_flag.clone()).await, Ok(()));
        assert_eq!(store.create(y_flag.clone()).await, Ok(()));
        assert_eq!(
            store.find_by_name("").await,
            Ok(vec![a_flag, t_flag, y_flag])
        );
    }

    async fn test_create_flag<C: Store<Flag>>(store: &mut C) {
        let flag = Flag::new_red(Uuid::new_v4(), "Test".to_string());

        // Should be able to create the flag once
        assert_eq!(store.create(flag.clone()).await, Ok(()));
        assert_eq!(Ok(flag.clone()), store.get_by_id(flag.id).await);

        // Should not be able to store a flag with the same name
        let flag_same_name = Flag::new_red(Uuid::new_v4(), "Test".to_string());
        assert_eq!(
            Err(StorageError::AlreadyExists),
            store.create(flag_same_name).await
        );

        // Should not be able to store a flag with the same id
        let flag_same_id = Flag {
            name: "Test".to_string(),
            ..flag
        };
        assert_eq!(
            Err(StorageError::AlreadyExists),
            store.create(flag_same_id).await
        );
    }
    async fn test_delete_by_id<C: Store<Flag>>(store: &mut C) {
        let flag = Flag::new_red(Uuid::new_v4(), "Test".to_string());
        assert_eq!(store.create(flag.clone()).await, Ok(()));
        assert_eq!(Ok(flag.clone()), store.get_by_id(flag.id).await);
        assert_eq!(store.delete_by_id(flag.id, Timestamp::now()).await, Ok(()));
        assert_eq!(Err(StorageError::NotFound), store.get_by_id(flag.id).await);
    }

    async fn test_get_for_flag<C: FlagStore>(store: &mut C) {
        let company1 = Uuid::new_v4();
        let company2 = Uuid::new_v4();
        let flag1 = Flag::new_red(company1, "Test 1".to_string());
        let flag2 = Flag::new_green(company1, "Test 2".to_string());
        let flag3 = Flag::new_red(company2, "Test 3".to_string());
        let flag4 = Flag::new_green(company2, "Test 4".to_string());
        assert_eq!(store.create(flag1.clone()).await, Ok(()));
        assert_eq!(store.create(flag2.clone()).await, Ok(()));
        assert_eq!(store.create(flag3.clone()).await, Ok(()));
        assert_eq!(store.create(flag4.clone()).await, Ok(()));
        assert_eq!(
            Ok(vec![flag1, flag2]),
            store.get_for_company(company1).await
        );
        assert_eq!(
            Ok(vec![flag3, flag4]),
            store.get_for_company(company2).await
        );
    }

    // Module for each implementation
    mod stub_flag_store {
        use crate::store::flag::stub_flag_store::StubFlagStore;

        #[tokio::test]
        async fn test_get_by_id() {
            let mut store = StubFlagStore::new();
            super::test_get_by_id(&mut store).await;
        }

        #[tokio::test]
        async fn test_get_by_name() {
            let mut store = StubFlagStore::new();
            super::test_get_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_find_by_name() {
            let mut store = StubFlagStore::new();
            super::test_find_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_create_flag() {
            let mut store = StubFlagStore::new();
            super::test_create_flag(&mut store).await;
        }

        #[tokio::test]
        async fn test_delete_by_id() {
            let mut store = StubFlagStore::new();
            super::test_delete_by_id(&mut store).await;
        }

        #[tokio::test]
        async fn test_get_for_flag() {
            let mut store = StubFlagStore::new();
            super::test_get_for_flag(&mut store).await;
        }
    }

    // Module for each implementation
    mod rocks_flag_store {
        use crate::RocksFlagStore;

        #[tokio::test]
        async fn test_get_by_id() {
            let mut store = RocksFlagStore::new_tmp().await.unwrap();
            super::test_get_by_id(&mut store).await;
        }

        #[tokio::test]
        async fn test_get_by_name() {
            let mut store = RocksFlagStore::new_tmp().await.unwrap();
            super::test_get_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_find_by_name() {
            let mut store = RocksFlagStore::new_tmp().await.unwrap();
            super::test_find_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_create_flag() {
            let mut store = RocksFlagStore::new_tmp().await.unwrap();
            super::test_create_flag(&mut store).await;
        }

        #[tokio::test]
        async fn test_delete_by_id() {
            let mut store = RocksFlagStore::new_tmp().await.unwrap();
            super::test_delete_by_id(&mut store).await;
        }

        #[tokio::test]
        async fn test_get_for_flag() {
            let mut store = RocksFlagStore::new_tmp().await.unwrap();
            super::test_get_for_flag(&mut store).await;
        }
    }

    // Module for each implementation
    mod libsql_flag_store {
        use super::*;

        #[tokio::test]
        async fn test_get_by_id() {
            let mut store = LibSqlFlagStore::new_tmp().await.unwrap();
            super::test_get_by_id(&mut store).await;
        }

        #[tokio::test]
        async fn test_get_by_name() {
            let mut store = LibSqlFlagStore::new_tmp().await.unwrap();
            super::test_get_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_find_by_name() {
            let mut store = LibSqlFlagStore::new_tmp().await.unwrap();
            super::test_find_by_name(&mut store).await;
        }

        #[tokio::test]
        async fn test_create_flag() {
            let mut store = LibSqlFlagStore::new_tmp().await.unwrap();
            super::test_create_flag(&mut store).await;
        }

        #[tokio::test]
        async fn test_delete_by_id() {
            let mut store = LibSqlFlagStore::new_tmp().await.unwrap();
            super::test_delete_by_id(&mut store).await;
        }

        #[tokio::test]
        async fn test_get_for_flag() {
            let mut store = LibSqlFlagStore::new_tmp().await.unwrap();
            super::test_get_for_flag(&mut store).await;
        }
    }

    mod flag {
        use super::*;

        #[test]
        fn test_get_name() {
            let flag = Flag::new_green(Uuid::new_v4(), "Test Flag".to_string());
            assert_eq!(flag.get_name(), flag.name);
        }
        #[test]
        fn test_get_id() {
            let flag = Flag::new_green(Uuid::new_v4(), "Test Flag".to_string());
            assert_eq!(flag.get_id(), flag.id);
        }
        #[test]
        fn test_get_and_set_deleted() {
            let mut flag = Flag::new_green(Uuid::new_v4(), "Test Flag".to_string());
            assert_eq!(flag.get_deleted(), None);
            let time = Timestamp::now();
            flag.set_deleted(time);
            assert_eq!(flag.get_deleted(), Some(time));
        }
    }
}

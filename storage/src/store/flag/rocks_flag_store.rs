use crate::store::{RocksStore, StorageError, Store};
use crate::{Flag, FlagStore, Timestamp};
use async_trait::async_trait;
use surrealdb::Response;
use uuid::Uuid;

pub type RocksFlagStore = RocksStore<Flag>;

const FLAG_TABLE_NAME: &str = "flag";

#[async_trait]
impl Store<Flag> for RocksFlagStore {
    async fn get_by_id(&self, id: Uuid) -> Result<Flag, StorageError> {
        let response: Response = self
            .db
            .query(
                "SELECT \
                   flag_id as id, \
                   company_id, \
                   name, \
                   flag_color, \
                   date_deleted \
                 FROM type::table($table) \
                 WHERE flag_id = $id \
                   AND date_deleted = None \
                 LIMIT 1;",
            )
            .bind(("table", FLAG_TABLE_NAME))
            .bind(("id", id))
            .await?;

        let mut checked_response = response.check()?;

        let companies: Option<Flag> = checked_response.take(0)?;

        companies.ok_or(StorageError::NotFound)
    }

    async fn get_by_name(&self, name: &str) -> Result<Flag, StorageError> {
        let response: Response = self
            .db
            .query(
                "SELECT \
                   flag_id as id, \
                   company_id, \
                   name, \
                   flag_color, \
                   date_deleted \
                 FROM type::table($table) \
                 WHERE name = $name \
                   AND date_deleted = None \
                 LIMIT 1;",
            )
            .bind(("table", FLAG_TABLE_NAME))
            .bind(("name", name.to_string()))
            .await?;

        let mut checked_response = response.check()?;

        let companies: Option<Flag> = checked_response.take(0)?;

        companies.ok_or(StorageError::NotFound)
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<Flag>, StorageError> {
        let response = self
            .db
            .query(
                "SELECT \
                   flag_id as id, \
                   company_id, \
                   name, \
                   flag_color, \
                   date_deleted \
                 FROM type::table($table) \
                 WHERE name ~ $name \
                 AND date_deleted = None;",
            )
            .bind(("table", FLAG_TABLE_NAME))
            .bind(("name", name.to_string()))
            .await?;

        let mut checked_response = response.check()?;

        Ok(checked_response.take(0)?)
    }

    async fn create(&mut self, flag: Flag) -> Result<(), StorageError> {
        if self.get_by_name(&flag.name).await.is_ok() {
            return Err(StorageError::AlreadyExists);
        }

        let response = self
            .db
            .query(
                "CREATE type::table($table) \
                 SET flag_id = $id, \
                     company_id = $company_id, \
                     name = $name, \
                     flag_color = $flag_color, \
                     date_deleted = $date_deleted",
            )
            .bind(("table", FLAG_TABLE_NAME))
            .bind(("id", flag.id))
            .bind(("company_id", flag.company_id))
            .bind(("name", flag.name))
            .bind(("flag_color", flag.flag_color))
            .bind(("date_deleted", flag.date_deleted))
            .await?;

        response.check()?;

        Ok(())
    }

    async fn delete_by_id(
        &mut self,
        id: Uuid,
        date_deleted: Timestamp,
    ) -> Result<(), StorageError> {
        let response = self
            .db
            .query("UPDATE type::table($table) SET date_deleted = $date_deleted WHERE flag_id = $id AND date_deleted = None;")
            .bind(("table", FLAG_TABLE_NAME))
            .bind(("id", id))
            .bind(("date_deleted", date_deleted))
            .await?;

        response.check()?;

        Ok(())
    }
}

#[async_trait]
impl FlagStore for RocksFlagStore {
    async fn get_for_company(&self, company_id: Uuid) -> Result<Vec<Flag>, StorageError> {
        let response = self
            .db
            .query(
                "SELECT \
                   flag_id as id, \
                   company_id, \
                   name, \
                   flag_color, \
                   date_deleted \
                 FROM type::table($table)\
                 WHERE company_id = $company_id \
                   AND date_deleted = None \
                 ORDER BY \
                   flag_color DESC, \
                   name ASC;",
            )
            .bind(("table", FLAG_TABLE_NAME))
            .bind(("company_id", company_id))
            .await?;

        let mut checked_response = response.check()?;

        Ok(checked_response.take(0)?)
    }
}

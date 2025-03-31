use crate::store::{RocksStore, StorageError, Store};
use crate::{Company, Timestamp};
use async_trait::async_trait;
use surrealdb::Response;
use uuid::Uuid;

pub type RocksCompanyStore = RocksStore<Company>;

const COMPANY_TABLE_NAME: &str = "company";

#[async_trait]
impl Store<Company> for RocksCompanyStore {
    async fn get_by_id(&self, id: Uuid) -> Result<Company, StorageError> {
        let response: Response = self
            .db
            .query(
                "SELECT \
                   company_id as id,\
                   name, \
                   date_deleted \
                 FROM type::table($table) \
                 WHERE company_id = $id \
                   AND date_deleted = None \
                 LIMIT 1;",
            )
            .bind(("table", COMPANY_TABLE_NAME))
            .bind(("id", id))
            .await?;

        let mut checked_response = response.check()?;

        let companies: Option<Company> = checked_response.take(0)?;

        companies.ok_or(StorageError::NotFound)
    }

    async fn get_by_name(&self, name: &str) -> Result<Company, StorageError> {
        let response: Response = self
            .db
            .query(
                "SELECT \
                   company_id as id, \
                   name, \
                   date_deleted \
                 FROM type::table($table) \
                 WHERE name = $name \
                   AND date_deleted = None \
                 LIMIT 1;",
            )
            .bind(("table", COMPANY_TABLE_NAME))
            .bind(("name", name.to_string()))
            .await?;

        let mut checked_response = response.check()?;

        let companies: Option<Company> = checked_response.take(0)?;

        companies.ok_or(StorageError::NotFound)
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<Company>, StorageError> {
        let response = self
            .db
            .query(
                "SELECT company_id as id, \
                   name, \
                   date_deleted \
                 FROM type::table($table) \
                 WHERE name ~ $name \
                   AND date_deleted = None;",
            )
            .bind(("table", COMPANY_TABLE_NAME))
            .bind(("name", name.to_string()))
            .await?;

        let mut checked_response = response.check()?;

        Ok(checked_response.take(0)?)
    }

    async fn create(&mut self, company: Company) -> Result<(), StorageError> {
        if self.get_by_name(&company.name).await.is_ok() {
            return Err(StorageError::AlreadyExists);
        }

        let response = self
            .db
            .query(
                "CREATE type::table($table) \
                 SET company_id = $id, \
                     name = $name, \
                     date_deleted = $date_deleted",
            )
            .bind(("table", COMPANY_TABLE_NAME))
            .bind(("id", company.id))
            .bind(("name", company.name))
            .bind(("date_deleted", company.date_deleted))
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
            .query("UPDATE type::table($table) SET date_deleted = $date_deleted WHERE company_id = $id AND date_deleted = None;")
            .bind(("table", COMPANY_TABLE_NAME))
            .bind(("id", id))
            .bind(("date_deleted", date_deleted))
            .await?;

        response.check()?;

        Ok(())
    }
}

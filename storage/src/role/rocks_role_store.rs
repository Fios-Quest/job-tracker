use crate::{Role, RoleStore, StorageError, Store, Timestamp};
use async_trait::async_trait;
use std::path::Path;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::{Response, Surreal};
use uuid::Uuid;

pub struct RocksRoleStore {
    db: Surreal<Db>,
}

impl RocksRoleStore {
    pub fn new(db: Surreal<Db>) -> Self {
        Self { db }
    }

    pub async fn new_from_path(path: &Path) -> Result<Self, StorageError> {
        let db = Surreal::new::<RocksDb>(path).await?;
        db.use_ns("test").use_db("test").await?;
        Ok(Self::new(db))
    }
}

const ROLE_TABLE_NAME: &str = "roles";

#[async_trait]
impl Store<Role> for RocksRoleStore {
    async fn get_by_id(&self, id: Uuid) -> Result<Role, StorageError> {
        let response: Response = self
            .db
            .query(
                "SELECT \
                   role_id as id, \
                   company_id, \
                   name, \
                   date_applied, \
                   date_deleted \
                 FROM type::table($table) \
                 WHERE role_id = $id \
                   AND date_deleted = None \
                 LIMIT 1;",
            )
            .bind(("table", ROLE_TABLE_NAME))
            .bind(("id", id))
            .await?;

        let mut checked_response = response.check()?;

        let companies: Option<Role> = checked_response.take(0)?;

        companies.ok_or_else(|| StorageError::NotFound)
    }

    async fn get_by_name(&self, name: &str) -> Result<Role, StorageError> {
        let response: Response = self
            .db
            .query(
                "SELECT \
                   role_id as id, \
                   company_id, \
                   name, \
                   date_applied, \
                   date_deleted \
                 FROM type::table($table) \
                 WHERE name = $name \
                   AND date_deleted = None \
                 LIMIT 1;",
            )
            .bind(("table", ROLE_TABLE_NAME))
            .bind(("name", name.to_string()))
            .await?;

        let mut checked_response = response.check()?;

        let companies: Option<Role> = checked_response.take(0)?;

        companies.ok_or_else(|| StorageError::NotFound)
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<Role>, StorageError> {
        let response = self
            .db
            .query(
                "SELECT \
                  role_id as id, \
                  company_id, \
                  name, \
                  date_applied, \
                  date_deleted \
                FROM type::table($table) \
                WHERE name ~ $name \
                  AND date_deleted = None;",
            )
            .bind(("table", ROLE_TABLE_NAME))
            .bind(("name", name.to_string()))
            .await?;

        let mut checked_response = response.check()?;

        Ok(checked_response.take(0)?)
    }

    async fn create(&mut self, role: Role) -> Result<(), StorageError> {
        if self.get_by_name(&role.name).await.is_ok() {
            return Err(StorageError::AlreadyExists);
        }

        let response = self
            .db
            .query(
                "CREATE type::table($table) \
                 SET role_id = $id, \
                     company_id = $company_id, \
                     name = $name, \
                     date_applied = $date_applied, \
                     date_deleted = $date_deleted",
            )
            .bind(("table", ROLE_TABLE_NAME))
            .bind(("id", role.id))
            .bind(("company_id", role.company_id))
            .bind(("name", role.name))
            .bind(("date_applied", role.date_applied))
            .bind(("date_deleted", role.date_deleted))
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
            .query(
                "UPDATE type::table($table) \
                 SET date_deleted = $date_deleted \
                 WHERE role_id = $id \
                   AND date_deleted = None;",
            )
            .bind(("table", ROLE_TABLE_NAME))
            .bind(("id", id))
            .bind(("date_deleted", date_deleted))
            .await?;

        response.check()?;

        Ok(())
    }
}

#[async_trait]
impl RoleStore for RocksRoleStore {
    async fn get_for_company(&self, company_id: Uuid) -> Result<Vec<Role>, StorageError> {
        let response = self
            .db
            .query(
                "SELECT \
                   role_id as id, \
                   company_id, \
                   name, \
                   date_applied, \
                   date_deleted \
                 FROM type::table($table) \
                 WHERE company_id = $company_id \
                   AND date_deleted = None \
                 ORDER BY \
                   date_applied ASC, \
                   name ASC;",
            )
            .bind(("table", ROLE_TABLE_NAME))
            .bind(("company_id", company_id))
            .await?;

        let mut checked_response = response.check()?;

        Ok(checked_response.take(0)?)
    }
}

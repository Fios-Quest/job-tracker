use crate::store::libsql::HasLibSqlTable;
use crate::{Company, LibSqlStore, StorageError, Store, Timestamp};
use async_trait::async_trait;
use libsql::params::Params;
use libsql::{de, params, Connection, Value};
use uuid::Uuid;

#[async_trait]
impl HasLibSqlTable for Company {
    async fn migrate(conn: &Connection) -> Result<(), StorageError> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS company ( \
                     id UUID PRIMARY KEY NOT NULL, \
                     name STRING NOT NULL, \
                     date_deleted INTEGER \
                 )",
            (),
        )
        .await?;
        Ok(())
    }
}

impl Company {
    fn into_params(self) -> Params {
        let mut params = Vec::with_capacity(3);
        params.push(Value::Text(self.id.to_string()));
        params.push(Value::Text(self.name));
        if let Some(date_deleted) = self.date_deleted {
            params.push(Value::Integer(date_deleted.into()));
        }
        Params::Positional(params)
    }
}

pub type LibSqlCompanyStore = LibSqlStore<Company>;

#[async_trait]
impl Store<Company> for LibSqlCompanyStore {
    async fn get_by_id(&self, id: Uuid) -> Result<Company, StorageError> {
        let mut rows = self
            .conn
            .query(
                "SELECT * FROM company WHERE id = ?1 AND date_deleted IS NULL LIMIT 1",
                [id.to_string()],
            )
            .await?;
        rows.next()
            .await?
            .ok_or(StorageError::NotFound)
            .and_then(|row| Ok(de::from_row::<Company>(&row)?))
    }

    async fn get_by_name(&self, name: &str) -> Result<Company, StorageError> {
        let mut rows = self
            .conn
            .query(
                "SELECT * FROM company WHERE name = ?1 AND date_deleted IS NULL LIMIT 1",
                [name],
            )
            .await?;
        rows.next()
            .await?
            .ok_or(StorageError::NotFound)
            .and_then(|row| Ok(de::from_row::<Company>(&row)?))
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<Company>, StorageError> {
        let pattern = format!("%{}%", name);
        let mut rows = self
            .conn
            .query(
                "SELECT * FROM company WHERE name LIKE ?1 AND date_deleted IS NULL ORDER BY name ASC",
                [pattern],
            )
            .await?;

        let mut companies = Vec::new();

        while let Some(row) = rows.next().await? {
            companies.push(de::from_row::<Company>(&row)?);
        }

        Ok(companies)
    }

    async fn create(&mut self, item: Company) -> Result<(), StorageError> {
        if self.get_by_name(&item.name).await.is_ok() {
            return Err(StorageError::AlreadyExists);
        }

        self.conn
            .execute(
                "INSERT INTO company (id, name, date_deleted) VALUES (?1, ?2, ?3)",
                item.into_params(),
            )
            .await?;
        Ok(())
    }

    async fn delete_by_id(
        &mut self,
        id: Uuid,
        date_deleted: Timestamp,
    ) -> Result<(), StorageError> {
        self.conn
            .execute(
                "UPDATE company SET date_deleted = ?2 WHERE id = ?1",
                params!(id.to_string(), i64::from(date_deleted)),
            )
            .await?;
        Ok(())
    }
}

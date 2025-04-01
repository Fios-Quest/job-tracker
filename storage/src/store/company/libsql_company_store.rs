use crate::store::libsql::HasLibSqlTable;
use crate::{Company, LibSqlStore, StorageError, Store, Timestamp};
use async_trait::async_trait;
use libsql::params::Params;
use libsql::{de, params, Connection, Row, Value};
use uuid::Uuid;

#[async_trait]
impl HasLibSqlTable for Company {
    async fn create_table_name(conn: &Connection) -> Result<(), StorageError> {
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

impl TryFrom<Row> for Company {
    type Error = StorageError;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value
                .get::<[u8; 16]>(0)
                .map(Uuid::from_bytes)
                .map_err(|_| StorageError::CouldNotMapToObject("company", format!("{value:?}")))?,
            name: value
                .get(1)
                .map_err(|_| StorageError::CouldNotMapToObject("company", format!("{value:?}")))?,
            date_deleted: value
                .get::<Option<i64>>(2)
                .map(|opt_timestamp| opt_timestamp.map(Timestamp::from))
                .map_err(|_| StorageError::CouldNotMapToObject("company", format!("{value:?}")))?,
        })
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
            .ok_or_else(|| StorageError::NotFound)
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
            .ok_or_else(|| StorageError::NotFound)
            .and_then(|row| Ok(de::from_row::<Company>(&row)?))
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<Company>, StorageError> {
        let pattern = format!("%{}%", name);
        let mut rows = self
            .conn
            .query(
                "SELECT * FROM company WHERE name LIKE ?1 AND date_deleted IS NULL LIMIT 1",
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

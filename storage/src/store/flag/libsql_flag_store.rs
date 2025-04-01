use crate::store::libsql::HasLibSqlTable;
use crate::{Flag, FlagStore, LibSqlStore, StorageError, Store, Timestamp};
use async_trait::async_trait;
use libsql::params::Params;
use libsql::{de, params, Connection, Value};
use uuid::Uuid;

#[async_trait]
impl HasLibSqlTable for Flag {
    async fn create_table_name(conn: &Connection) -> Result<(), StorageError> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS flag ( \
                     id UUID PRIMARY KEY NOT NULL, \
                     company_id UUID NOT NULL, \
                     flag_color STRING NOT NULL, \
                     name STRING NOT NULL, \
                     date_deleted INTEGER \
                 )",
            (),
        )
        .await?;
        Ok(())
    }
}

impl Flag {
    fn into_params(self) -> Params {
        let mut params = Vec::with_capacity(3);
        params.push(Value::Text(self.id.to_string()));
        params.push(Value::Text(self.company_id.to_string()));
        params.push(Value::Text(self.flag_color.to_string()));
        params.push(Value::Text(self.name));
        if let Some(date_deleted) = self.date_deleted {
            params.push(Value::Integer(date_deleted.into()));
        }
        Params::Positional(params)
    }
}

pub type LibSqlFlagStore = LibSqlStore<Flag>;

#[async_trait]
impl Store<Flag> for LibSqlFlagStore {
    async fn get_by_id(&self, id: Uuid) -> Result<Flag, StorageError> {
        let mut rows = self
            .conn
            .query(
                "SELECT * FROM flag WHERE id = ?1 AND date_deleted IS NULL LIMIT 1",
                [id.to_string()],
            )
            .await?;
        rows.next()
            .await?
            .ok_or_else(|| StorageError::NotFound)
            .and_then(|row| Ok(de::from_row::<Flag>(&row)?))
    }

    async fn get_by_name(&self, name: &str) -> Result<Flag, StorageError> {
        let mut rows = self
            .conn
            .query(
                "SELECT * FROM flag WHERE name = ?1 AND date_deleted IS NULL LIMIT 1",
                [name],
            )
            .await?;
        rows.next()
            .await?
            .ok_or_else(|| StorageError::NotFound)
            .and_then(|row| Ok(de::from_row::<Flag>(&row)?))
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<Flag>, StorageError> {
        let pattern = format!("%{}%", name);
        let mut rows = self
            .conn
            .query(
                "SELECT * FROM flag WHERE name LIKE ?1 AND date_deleted IS NULL ORDER BY name ASC",
                [pattern],
            )
            .await?;

        let mut flags = Vec::new();

        while let Some(row) = rows.next().await? {
            flags.push(de::from_row::<Flag>(&row)?);
        }

        Ok(flags)
    }

    async fn create(&mut self, item: Flag) -> Result<(), StorageError> {
        if self.get_by_name(&item.name).await.is_ok() {
            return Err(StorageError::AlreadyExists);
        }

        self.conn
            .execute(
                "INSERT INTO flag (id, company_id, flag_color, name, date_deleted) VALUES (?1, ?2, ?3, ?4, ?5)",
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
                "UPDATE flag SET date_deleted = ?2 WHERE id = ?1",
                params!(id.to_string(), i64::from(date_deleted)),
            )
            .await?;
        Ok(())
    }
}

#[async_trait]
impl FlagStore for LibSqlFlagStore {
    async fn get_for_company(&self, company_id: Uuid) -> Result<Vec<Flag>, StorageError> {
        let mut rows = self
            .conn
            .query(
                "SELECT * FROM flag WHERE company_id = ?1 AND date_deleted IS NULL;",
                [company_id.to_string()],
            )
            .await?;

        let mut flags = Vec::new();

        while let Some(row) = rows.next().await? {
            flags.push(de::from_row::<Flag>(&row)?);
        }

        Ok(flags)
    }
}

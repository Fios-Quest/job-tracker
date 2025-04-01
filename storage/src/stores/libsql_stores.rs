use crate::store::{HasLibSqlTable, StorageError};
use crate::{Company, Flag, LibSqlCompanyStore, LibSqlFlagStore, LibSqlRoleStore, Role, Stores};
use libsql::Builder;
use std::path::PathBuf;

pub struct LibSqlStores {
    company_store: LibSqlCompanyStore,
    role_store: LibSqlRoleStore,
    flag_store: LibSqlFlagStore,
}

impl LibSqlStores {
    pub async fn new(path_to_db: PathBuf) -> Result<Self, StorageError> {
        let conn = Builder::new_local(path_to_db).build().await?.connect()?;

        Company::create_table_name(&conn).await?;
        Role::create_table_name(&conn).await?;
        Flag::create_table_name(&conn).await?;

        Ok(Self {
            company_store: LibSqlCompanyStore::new(conn.clone()),
            role_store: LibSqlRoleStore::new(conn.clone()),
            flag_store: LibSqlFlagStore::new(conn.clone()),
        })
    }
}

impl Stores<LibSqlCompanyStore, LibSqlRoleStore, LibSqlFlagStore> for LibSqlStores {
    fn company_store(&mut self) -> &mut LibSqlCompanyStore {
        &mut self.company_store
    }

    fn role_store(&mut self) -> &mut LibSqlRoleStore {
        &mut self.role_store
    }

    fn flag_store(&mut self) -> &mut LibSqlFlagStore {
        &mut self.flag_store
    }
}

mod application_context;
pub use application_context::*;

mod time;
pub use time::*;

mod error;
pub use error::*;

mod composite_store;
mod storable;
mod storage;

#[cfg(test)]
mod test_helper;

// prevent traits being externally implemented
trait Sealed {}

pub mod prelude {
    pub use crate::application_context::{ApplicationContext, ApplicationContextError};
    pub use crate::composite_store::{HasFutureStoreFor, ThreadSafeGeneralStore};
    pub use crate::error::StorageError;
    pub use crate::storable::{
        Company, Flag, FlagColor, HasCompany, HasDeleted, HasId, HasName, Role,
    };
    pub use crate::storage::{
        BaseStore, CompanyStore, FlagStore, JsonStore, RecallByCompany, RecallById, RecallByName,
        RoleStore, ScopedJsonStoreFor,
    };
    pub use crate::time::Timestamp;
}

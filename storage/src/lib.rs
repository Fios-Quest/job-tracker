mod application_context;
pub use application_context::*;

mod time;
pub use time::*;

mod error;
pub use error::*;

mod composite_store;
mod storable;
mod storage;

// prevent traits being externally implemented
trait Sealed {}

pub mod prelude {
    use super::*;

    pub use application_context::{ApplicationContext, ApplicationContextError};
    pub use composite_store::{HasFutureStoreFor, ThreadSafeGeneralStore};
    pub use error::StorageError;
    pub use storable::{Company, Flag, FlagColor, HasCompany, HasDeleted, HasId, HasName, Role};
    pub use storage::{
        BaseStore, CompanyStore, FlagStore, RecallByCompany, RecallById, RecallByName, RoleStore,
    };
    pub use time::Timestamp;
}

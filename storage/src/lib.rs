// ToDo: Can we do a better job with this code?
// - async_fn_in_trait <- explicit future types seem to not like future in future as future is not
//   Send
// - private_bounds <- Need to figure out how to prevent conflicting implementations of
//   HasFutureStoreFor
#![allow(async_fn_in_trait, private_bounds)]

mod application_context;
pub use application_context::*;

mod time;
pub use time::*;

mod error;
pub use error::*;

mod composite_store;
mod storable;
mod storage;

mod logging;

#[cfg(test)]
mod test_helper;

// prevent traits being externally implemented
trait Sealed {}

pub mod prelude {
    pub use crate::application_context::{ApplicationContext, ApplicationContextError};
    pub use crate::composite_store::{
        HasFutureStoreFor, JsonThreadSafeGeneralStore, ThreadSafeGeneralStore,
    };
    pub use crate::error::StorageError;
    pub use crate::logging::{json_log_fetcher::JsonLogFetcher, LogFetcher};
    pub use crate::storable::{
        ApplyPartial, Company, Flag, FlagColor, HasCompany, HasDeleted, HasId, HasName, HasRole,
        Interview, PartialCompany, PartialFlag, PartialInterview, PartialQuestion, PartialRole,
        PartialValue, Question, Role, Value,
    };
    pub use crate::storage::{
        BaseStore, CompanyStore, FlagStore, JsonStore, RecallByCompany, RecallById, RecallByName,
        RecallByRole, RoleStore, ScopedJsonStoreFor,
    };
    pub use crate::time::Timestamp;
}

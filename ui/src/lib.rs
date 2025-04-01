//! This crate contains all shared UI for the workspace.
use std::sync::Arc;
use tokio::sync::Mutex;

mod hero;
pub use hero::Hero;

mod navbar;
pub use navbar::Navbar;

mod echo;
pub use echo::Echo;

mod company_list;
pub use company_list::CompanyList;

mod role_list;
pub use role_list::*;

mod flag_list;
pub use flag_list::*;

#[cfg(feature = "desktop")]
type StoreType = storage::LibSqlStores;
type StoreContext = Arc<Mutex<StoreType>>;

//! This crate contains all shared UI for the workspace.

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

mod role_information;
pub use role_information::*;

mod flag_list;
pub use flag_list::*;

mod editable;
pub use editable::*;

mod error_message;

use storage::prelude::*;
#[cfg(feature = "desktop")]
pub type StoreType = JsonThreadSafeGeneralStore;

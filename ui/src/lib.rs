//! This crate contains all shared UI for the workspace.

mod router;
pub use router::Route;

mod navbar;
pub use navbar::Navbar;

mod company_list;
pub use company_list::CompanyList;

mod company_details;
pub use company_details::*;

mod role_list;
pub use role_list::*;

mod role_information;
pub use role_information::*;

mod flag_list;
pub use flag_list::*;

mod editable;
pub use editable::*;

mod error_message;
pub mod views;

use storage::prelude::*;
#[cfg(feature = "desktop")]
pub type StoreType = JsonThreadSafeGeneralStore;

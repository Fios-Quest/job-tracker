//! This crate contains all shared UI for the workspace.

mod router;
pub use router::{DetailsView, Route};

mod navbar;
pub use navbar::Navbar;

mod interviews;
pub use interviews::*;

mod company_list;
pub use company_list::CompanyList;

mod role_list;
pub use role_list::*;

mod role_information;
pub use role_information::*;

mod flag_list;
pub use flag_list::*;

mod value_list;
pub use value_list::*;

mod questions_list;
pub use questions_list::*;

mod editable;
pub use editable::*;

mod error_message;

mod main_nav;
pub use main_nav::*;

mod views;
pub use views::*;

use dioxus::prelude::*;
use storage::prelude::*;

#[cfg(feature = "desktop")]
pub type LogFetcherType = JsonLogFetcher;

#[cfg(feature = "desktop")]
pub type StoreType = JsonThreadSafeGeneralStore;

pub static VIEW_SIGNAL: GlobalSignal<Option<DetailsView>> = Global::new(|| None);

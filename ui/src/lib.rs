//! This crate contains all shared UI for the workspace.

mod hero;
pub use hero::Hero;

mod navbar;
pub use navbar::Navbar;

mod echo;
pub use echo::Echo;

mod company_list;
pub use company_list::CompanyList;

mod roles_list;
pub use roles_list::EmptyRolesList;
pub use roles_list::RolesList;
use storage::{Stores, StubCompanyStore, StubFlagStore, StubRoleStore};

#[cfg(feature = "desktop")]
type StoreContext = Stores<StubCompanyStore, StubRoleStore, StubFlagStore>;

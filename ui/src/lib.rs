//! This crate contains all shared UI for the workspace.

use anyhow::Result;
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

mod role_information;
pub use role_information::*;

mod flag_list;
pub use flag_list::*;

mod editable;
pub use editable::*;
use storage::storable::object::company::Company;
use storage::storable::object::flag::Flag;
use storage::storable::object::role::Role;
use storage::{FlagStore, GetId, RoleStore, Store, Stores};

mod error_message;

#[cfg(feature = "desktop")]
type StoreType = storage::JsonStores;

#[derive(Clone)]
pub struct StoreContext(Arc<Mutex<StoreType>>);

impl StoreContext {
    pub fn new(store: StoreType) -> Self {
        Self(Arc::new(Mutex::new(store)))
    }

    // --- Companies ---
    pub(crate) async fn find_company_by_name(&self, name: &str) -> Result<Vec<Company>> {
        self.0.lock().await.company_store().find_by_name(name).await
    }

    pub(crate) async fn create_company(&self, company: &Company) -> Result<()> {
        self.0.lock().await.company_store().create(company).await
    }

    pub(crate) async fn update_company(&self, company: &Company) -> Result<()> {
        self.0.lock().await.company_store().update(company).await
    }

    // --- Flags ---
    pub(crate) async fn get_flags_for_company<I: GetId>(&self, company: I) -> Result<Vec<Flag>> {
        self.0
            .lock()
            .await
            .flag_store()
            .get_for_company(company.get_id())
            .await
    }

    pub(crate) async fn create_flag(&self, flag: &Flag) -> Result<()> {
        self.0.lock().await.flag_store().create(flag).await
    }

    pub(crate) async fn update_flag(&self, flag: &Flag) -> Result<()> {
        self.0.lock().await.flag_store().update(flag).await
    }

    // --- Roles ---
    pub(crate) async fn get_role<I: GetId>(&self, role: I) -> Result<Role> {
        self.0
            .lock()
            .await
            .role_store()
            .get_by_id(role.get_id())
            .await
    }

    pub(crate) async fn get_roles_for_company<I: GetId>(&self, company: I) -> Result<Vec<Role>> {
        self.0
            .lock()
            .await
            .role_store()
            .get_for_company(company.get_id())
            .await
    }

    pub(crate) async fn create_role(&self, role: &Role) -> Result<()> {
        self.0.lock().await.role_store().create(role).await
    }

    pub(crate) async fn update_role(&self, role: &Role) -> Result<()> {
        self.0.lock().await.role_store().update(role).await
    }
}

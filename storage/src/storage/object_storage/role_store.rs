use crate::storable::object::role::Role;
use crate::storage::property::base_store::BaseStore;
use crate::storage::property::recall_by_company::RecallByCompany;
use crate::storage::property::recall_by_name::RecallByName;

pub trait RoleStore: BaseStore<Role> + RecallByName<Role> + RecallByCompany<Role> {}

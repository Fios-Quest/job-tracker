use crate::storable::Role;
use crate::storage::{BaseStore, RecallByCompany, RecallByName};

pub trait RoleStore: BaseStore<Role> + RecallByName<Role> + RecallByCompany<Role> {}

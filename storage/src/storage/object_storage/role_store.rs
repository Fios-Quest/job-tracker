use crate::prelude::RecallById;
use crate::storable::Role;
use crate::storage::{BaseStore, RecallByCompany, RecallByName};

pub trait RoleStore:
    BaseStore<Role> + RecallById<Role> + RecallByName<Role> + RecallByCompany<Role>
{
}

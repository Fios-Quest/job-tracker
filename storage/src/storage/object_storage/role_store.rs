use crate::storable::Role;
use crate::storage::*;

pub trait RoleStore:
    BaseStore<Role> + RecallById<Role> + RecallByName<Role> + RecallByCompany<Role>
{
}

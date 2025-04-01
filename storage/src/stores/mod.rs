use crate::{Company, FlagStore, RoleStore};

mod stub_stores;
pub use stub_stores::*;

mod rocks_stores;
pub use rocks_stores::*;

mod libsql_stores;
pub use libsql_stores::*;

use crate::store::Store;

pub trait Stores<C, R, F>
where
    C: Store<Company>,
    R: RoleStore,
    F: FlagStore,
{
    fn company_store(&mut self) -> &mut C;

    fn role_store(&mut self) -> &mut R;

    fn flag_store(&mut self) -> &mut F;
}

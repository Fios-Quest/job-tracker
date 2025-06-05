use crate::storage::{CompanyStore, FlagStore, RoleStore};
use crate::Sealed;

// Note: General Store can not be as a general store (I know 🙄) used like Thread Safe General Store
// due to conflicting implementations for the Storage properties
pub struct GeneralStore<C, F, R>
where
    C: CompanyStore,
    F: FlagStore,
    R: RoleStore,
{
    company_store: C,
    flag_store: F,
    role_store: R,
}

impl<C, F, R> GeneralStore<C, F, R>
where
    C: CompanyStore,
    F: FlagStore,
    R: RoleStore,
{
    pub fn new(company_store: C, flag_store: F, role_store: R) -> Self {
        Self {
            company_store,
            flag_store,
            role_store,
        }
    }

    pub fn company_store(&mut self) -> &mut C {
        &mut self.company_store
    }

    pub fn flag_store(&mut self) -> &mut F {
        &mut self.flag_store
    }

    pub fn role_store(&mut self) -> &mut R {
        &mut self.role_store
    }
}

impl<C, F, R> Sealed for GeneralStore<C, F, R>
where
    C: CompanyStore,
    F: FlagStore,
    R: RoleStore,
{
}

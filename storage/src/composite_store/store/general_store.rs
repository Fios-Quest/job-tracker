// use crate::composite_store::{HasMutStoreFor, HasStoreFor};
use crate::storage::{CompanyStore, FlagStore, RoleStore};
use crate::Sealed;

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

    pub fn company_store(&self) -> &C {
        &self.company_store
    }

    pub fn flag_store(&self) -> &F {
        &self.flag_store
    }

    pub fn role_store(&self) -> &R {
        &self.role_store
    }

    pub fn company_store_mut(&mut self) -> &mut C {
        &mut self.company_store
    }

    pub fn flag_store_mut(&mut self) -> &mut F {
        &mut self.flag_store
    }

    pub fn role_store_mut(&mut self) -> &mut R {
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

// impl<C, F, R> HasStoreFor<Company> for GeneralStore<C, F, R>
// where
//     C: CompanyStore,
//     F: FlagStore,
//     R: RoleStore,
// {
//     type Storage = C;
//
//     fn get_store(&self) -> &Self::Storage {
//         self.company_store()
//     }
// }
//
// impl<C, F, R> HasMutStoreFor<Company> for GeneralStore<C, F, R>
// where
//     C: CompanyStore,
//     F: FlagStore,
//     R: RoleStore,
// {
//     type Storage = C;
//
//     fn get_mut_store(&mut self) -> &mut Self::Storage {
//         self.company_store_mut()
//     }
// }
//
// impl<C, F, R> HasStoreFor<Flag> for GeneralStore<C, F, R>
// where
//     C: CompanyStore,
//     F: FlagStore,
//     R: RoleStore,
// {
//     type Storage = F;
//
//     fn get_store(&self) -> &Self::Storage {
//         self.flag_store()
//     }
// }
//
// impl<C, F, R> HasMutStoreFor<Flag> for GeneralStore<C, F, R>
// where
//     C: CompanyStore,
//     F: FlagStore,
//     R: RoleStore,
// {
//     type Storage = F;
//
//     fn get_mut_store(&mut self) -> &mut Self::Storage {
//         self.flag_store_mut()
//     }
// }
//
// impl<C, F, R> HasStoreFor<Role> for GeneralStore<C, F, R>
// where
//     C: CompanyStore,
//     F: FlagStore,
//     R: RoleStore,
// {
//     type Storage = R;
//
//     fn get_store(&self) -> &Self::Storage {
//         self.role_store()
//     }
// }
// impl<C, F, R> HasMutStoreFor<Role> for GeneralStore<C, F, R>
// where
//     C: CompanyStore,
//     F: FlagStore,
//     R: RoleStore,
// {
//     type Storage = R;
//
//     fn get_mut_store(&mut self) -> &mut Self::Storage {
//         self.role_store_mut()
//     }
// }

#[cfg(test)]
mod tests {

    // #[tokio::test]
    // async fn test_base_store() {
    //     let company = Company::new("name");
    //     let flag = Flag::new_green(company.id, "good".to_string());
    //     let role = Role::new(company.id, "role".to_string(), Timestamp::now());
    //
    //     let mut all_store = GeneralStore {
    //         company_store: StubStore::default(),
    //         flag_store: StubStore::default(),
    //         role_store: StubStore::default(),
    //     };
    //
    //     all_store.store(company.clone()).await.unwrap();
    //     all_store.store(flag.clone()).await.unwrap();
    //     all_store.store(role.clone()).await.unwrap();
    //
    //     let recalled_company: Company = all_store.recall_by_id(&company).await.unwrap();
    //     let recalled_flag: Flag = all_store.recall_by_id(&flag).await.unwrap();
    //     let recalled_role: Role = all_store.recall_by_id(&role).await.unwrap();
    //
    //     assert_eq!(recalled_company, company);
    //     assert_eq!(recalled_flag, flag);
    //     assert_eq!(recalled_role, role);
    // }
    //
    // #[tokio::test]
    // async fn test_recall_by_name() {
    //     let company = Company::new("name");
    //     let flag = Flag::new_green(company.id, "good".to_string());
    //     let role = Role::new(company.id, "role".to_string(), Timestamp::now());
    //
    //     let mut all_store = GeneralStore {
    //         company_store: StubStore::default(),
    //         flag_store: StubStore::default(),
    //         role_store: StubStore::default(),
    //     };
    //
    //     all_store.store(company.clone()).await.unwrap();
    //
    //     let recalled_companies: Vec<Company> = all_store.recall_by_name(&company).await.unwrap();
    //
    //     assert!(recalled_companies.contains(&company));
    // }
    //
    // #[tokio::test]
    // async fn test_recall_by_company() {
    //     let company = Company::new("name");
    //     let flag = Flag::new_green(company.id, "good".to_string());
    //     let role = Role::new(company.id, "role".to_string(), Timestamp::now());
    //
    //     let mut all_store = GeneralStore {
    //         company_store: StubStore::default(),
    //         flag_store: StubStore::default(),
    //         role_store: StubStore::default(),
    //     };
    //
    //     all_store.store(flag.clone()).await.unwrap();
    //     all_store.store(role.clone()).await.unwrap();
    //
    //     let recalled_flags: Vec<Flag> = all_store.recall_by_company(&company).await.unwrap();
    //     let recalled_roles: Vec<Role> = all_store.recall_by_company(&company).await.unwrap();
    //
    //     assert!(recalled_flags.contains(&flag));
    //     assert!(recalled_roles.contains(&role));
    // }
}

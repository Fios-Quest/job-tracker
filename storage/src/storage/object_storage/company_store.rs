use crate::storable::object::company::Company;
use crate::storage::property::base_store::BaseStore;
use crate::storage::property::recall_by_name::RecallByName;

pub trait CompanyStore: BaseStore<Company> + RecallByName<Company> {}

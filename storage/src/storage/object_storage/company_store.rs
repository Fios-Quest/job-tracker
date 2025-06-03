use crate::storable::Company;
use crate::storage::{BaseStore, RecallByName};

pub trait CompanyStore: BaseStore<Company> + RecallByName<Company> {}

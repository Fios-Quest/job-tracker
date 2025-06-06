use crate::storable::Company;
use crate::storage::{BaseStore, RecallById, RecallByName};

pub trait CompanyStore: BaseStore<Company> + RecallById<Company> + RecallByName<Company> {}

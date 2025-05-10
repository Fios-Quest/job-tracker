use crate::store::json::JsonStore;
use crate::{Company, StubCompanyStore};

pub type JsonCompanyStore = JsonStore<Company, StubCompanyStore>;

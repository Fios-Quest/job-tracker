use crate::store::json::{JsonStore, JsonStoreConstructor};
use crate::{Company, StubCompanyStore};

pub type JsonCompanyStore = JsonStore<Company>;

impl JsonStoreConstructor<Company> for JsonCompanyStore {
    fn create_stub_store() -> StubCompanyStore {
        StubCompanyStore::new()
    }
}

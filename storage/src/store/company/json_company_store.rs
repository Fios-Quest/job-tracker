use crate::store::json::{JsonStore, JsonStoreConstructor};
use crate::{Company, StubCompanyStore};

pub type JsonCompanyStore = JsonStore<Company, StubCompanyStore>;

impl JsonStoreConstructor<Company, StubCompanyStore> for JsonCompanyStore {
    fn create_stub_store() -> StubCompanyStore {
        StubCompanyStore::new()
    }
}

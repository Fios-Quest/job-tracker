use dioxus::prelude::*;
use storage::{Company, Store, StubCompanyStore};
use ui::{CompanyList, Echo, Hero};

#[component]
pub fn Home() -> Element {
    rsx! {
        CompanyList {}
    }
}

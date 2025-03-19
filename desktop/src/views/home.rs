use dioxus::prelude::*;
use ui::CompanyList;

#[component]
pub fn Home() -> Element {
    rsx! {
        CompanyList {}
    }
}

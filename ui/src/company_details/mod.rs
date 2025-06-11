use crate::PopulatedFlagList;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::Company;

#[component]
pub fn CompanyDetails(company: Arc<Company>) -> Element {
    rsx! {
        h1 { class: "text-slate-200 text-3xl", "Company Details" }
        PopulatedFlagList { company }
    }
}

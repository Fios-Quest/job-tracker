use super::flag_list::PopulatedFlagList;
use super::value_list::ValueList;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn CompanyDetails(company: Arc<Company>) -> Element {
    rsx! {
        h2 { {company.get_name()} }
        ValueList { company: company.clone() }
        PopulatedFlagList { company }
    }
}

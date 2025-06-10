use crate::FlagList;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn CompanyDetails(company_id: Option<Uuid>) -> Element {
    rsx! {
        h1 { class: "text-slate-200 text-3xl", "Company Details" }
        if let Some(company_id) = company_id {
            FlagList { company_id }
        } else {
            p { "Add or select a company to see their details" }
        }
    }
}

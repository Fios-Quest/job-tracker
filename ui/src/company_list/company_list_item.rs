use dioxus::prelude::*;
use storage::{ApplicationContext, Company};

#[component]
pub fn CompanyListItem(company: Company) -> Element {
    let mut application_context = use_context::<Signal<ApplicationContext>>();
    let Company { id, name, .. } = company;

    rsx! {
        li { key: id,
            input {
                id: id.to_string(),
                r#type: "radio",
                name: "company",
                onchange: move |_| application_context.set(application_context().set_company_id(id)),
            }
            label { r#for: id.to_string(), "{name}" }
        }
    }
}

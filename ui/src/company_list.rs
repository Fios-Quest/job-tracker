use dioxus::prelude::*;
use std::sync::{Arc, Mutex};
use storage::{Company, Store, StubCompanyStore};

#[component]
pub fn CompanyList() -> Element {
    let company_store = use_context::<Arc<Mutex<StubCompanyStore>>>();
    let mut company_name_value = use_signal(|| "");

    let mut companies_items = use_resource(move || async move {
        use_context::<Arc<Mutex<StubCompanyStore>>>()
            .lock()
            .expect("Could not lock company store")
            .find_by_name("")
            .await
            .expect("Did not get companies")
    });
    let list = companies_items().unwrap_or_default();
    let companies = list.iter().map(|c| {
        let name = &c.name;
        rsx! {
            li {
                key: c.id,
                "{name}"
            }
        }
    });

    let onsubmit = move |event: Event<FormData>| {
        let company_store = company_store.clone();
        async move {
            let company_name = event.values().get("company_name").map(|v| v.as_value());

            if let Some(company_name) = company_name {
                if !company_name.is_empty() {
                    // Store the name
                    company_store
                        .lock()
                        .expect("Could not lock company store")
                        .create(Company::new(company_name))
                        .await
                        .expect("Could not store new company");

                    companies_items.restart();

                    // Reset the value to empty
                    company_name_value.set("")
                }
            }
        }
    };

    rsx! {
        div {
            h3 { "Companies" }
            ul {
                { companies }
            }
            form {
                onsubmit,
                input { id: "add_company", name: "company_name", value: company_name_value }
                input { r#type: "submit" }
            }
        }
    }
}

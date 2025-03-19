use dioxus::prelude::*;
use std::sync::{Arc, Mutex};
use storage::{Company, Store, StubCompanyStore};

#[component]
pub fn CompanyList() -> Element {
    let company_store = use_context::<Arc<Mutex<StubCompanyStore>>>();
    let mut company_name_value = use_signal(|| "");
    let mut company_name_search = use_signal(|| "".to_string());

    let mut companies_items = use_resource(move || async move {
        let search = company_name_search();
        use_context::<Arc<Mutex<StubCompanyStore>>>()
            .lock()
            .expect("Could not lock company store")
            .find_by_name(&search)
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

    let create_company = move |event: Event<FormData>| {
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

                    // Reset the values to empty
                    company_name_value.set("");
                    company_name_search.set("".to_string());

                    // Rerun the resource
                    companies_items.restart();
                }
            }
        }
    };

    let company_search = move |event: Event<FormData>| {
        let search = event.value();
        company_name_search.set(search);
    };

    rsx! {
        div {
            h3 { "Companies" }

            input { id: "search_companies", value: company_name_search, onchange: company_search, }

            ul {
                { companies }
            }

            form {
                onsubmit: create_company,
                input { id: "add_company", name: "company_name", value: company_name_value }
                input { r#type: "submit" }
            }
        }
    }
}

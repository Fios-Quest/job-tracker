mod company_list_item;

use crate::StoreContext;
use company_list_item::CompanyListItem;
use dioxus::prelude::*;
use std::sync::{Arc, Mutex};
use storage::{Company, Store, Stores};

#[component]
pub fn CompanyList() -> Element {
    let stores = use_context::<Arc<Mutex<StoreContext>>>();
    let mut company_name_value = use_signal(|| "");
    let mut company_name_search = use_signal(|| "".to_string());

    let mut companies_resource = use_resource(move || async move {
        let search = company_name_search();
        use_context::<Arc<Mutex<StoreContext>>>()
            .lock()
            .expect("Could not lock company store")
            .company_store()
            .find_by_name(&search)
            .await
            .expect("Did not get companies")
    });
    let companies = companies_resource().unwrap_or_default();
    let companies_list = companies.iter().cloned().map(|company| {
        rsx! {
            CompanyListItem { company }
        }
    });

    let create_company = move |event: Event<FormData>| {
        let stores = stores.clone();
        async move {
            let company_name = event.values().get("company_name").map(|v| v.as_value());

            if let Some(company_name) = company_name {
                if !company_name.is_empty() {
                    // Store the name
                    let mut stores_lock = stores.lock().expect("Could not lock company store");
                    stores_lock
                        .company_store()
                        .create(Company::new(company_name))
                        .await
                        .expect("Could not store new company");

                    // Reset the values to empty
                    company_name_value.set("");
                    company_name_search.set("".to_string());

                    // Rerun the resource
                    companies_resource.restart();
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
            id: "companies",

            h3 { "Companies" }

            input { id: "search_companies", value: company_name_search, onchange: company_search, }

            ul {
                { companies_list }
            }

            form {
                onsubmit: create_company,
                input { id: "add_company", name: "company_name", value: company_name_value }
                input { r#type: "submit" }
            }
        }
    }
}

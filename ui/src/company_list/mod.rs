mod company_list_item;

use crate::error_message::ErrorMessage;
use crate::StoreType;
use company_list_item::CompanyListItem;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use storage::prelude::*;

fn handle_storage_error(error: anyhow::Error) -> Option<String> {
    tracing::error!("Storage Error: {:?}", error);

    match error.downcast_ref::<StorageError>() {
        Some(StorageError::NotFound) => Some("No company found".to_string()),
        Some(StorageError::AlreadyExists) => Some("Company already exists".to_string()),
        _ => Some("A database error has occurred".to_string()),
    }
}

#[component]
pub fn CompanyList() -> Element {
    let stores = use_context::<StoreType>();
    let mut company_name_value = use_signal(|| "");
    let mut company_name_search = use_signal(|| "".to_string());
    let mut error_message = use_signal(|| None);

    let mut companies_resource = use_resource(move || async move {
        let search = company_name_search();
        let companies = use_context::<StoreType>().recall_by_name(search).await;
        match companies {
            Ok(companies) => companies,
            Err(e) => {
                error_message.set(handle_storage_error(e));
                Vec::with_capacity(0)
            }
        }
    });
    let reload_companies = use_callback(move |()| companies_resource.restart());
    let companies = companies_resource().unwrap_or_default();
    let companies_list = companies.into_iter().map(|company| {
        rsx! {
            CompanyListItem { company, reload_companies }
        }
    });

    let create_company = move |event: Event<FormData>| {
        let mut stores = stores.clone();
        error_message.set(None);
        let company_name = event.values().get("company_name").map(|v| v.as_value());

        async move {
            if let Some(company_name) = company_name {
                if !company_name.is_empty() {
                    // Store the name
                    let store_result = stores.store(Company::new(company_name)).await;

                    match store_result {
                        Ok(()) => {
                            // Reset the values to empty
                            company_name_value.set("");
                            company_name_search.set("".to_string());
                            error_message.set(None);

                            // Rerun the resource
                            companies_resource.restart();
                        }
                        Err(e) => {
                            error_message.set(handle_storage_error(e));
                        }
                    }
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
            header { class: "text-2xl",
                p { "Companies" }
            }

            ul { {companies_list} }

            if let Some(message) = error_message() {
                ErrorMessage { message }
            }

            form { class: "flex flex-col", onsubmit: create_company,
                input {
                    id: "add_company",
                    class: "bg-slate-800 text-slate-200 rounded",
                    name: "company_name",
                    value: company_name_search,
                    oninput: company_search,
                }
                input {
                    class: "bg-slate-400 text-slate-900 rounded cursor-pointer m-auto py-1.5 px-4",
                    r#type: "submit",
                }
            }
        }
    }
}

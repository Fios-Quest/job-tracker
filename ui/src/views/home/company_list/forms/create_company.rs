use crate::helpers::log_error;
use crate::StoreType;
use dioxus::prelude::*;
use storage::prelude::{BaseStore, Company, CompanyFieldName, PartialCompany};

fn create_on_submit(callback: Callback<Company>) -> impl FnMut(FormEvent) {
    move |e: FormEvent| {
        e.prevent_default();
        if let Ok(company) = e
            .parsed_values::<PartialCompany>()
            .map_err(log_error)
            .and_then(|form_data| Company::new_from_partial(form_data).map_err(log_error))
        {
            // If the company was successfully created, save it
            spawn(async move {
                let mut stores = use_context::<StoreType>();
                stores
                    .store(company.clone())
                    .await
                    .unwrap_or_else(log_error);
                callback(company);
            });
        }
    }
}

#[component]
pub fn CreateCompany(
    callback: Callback<Company>,
    company_search: Callback<Event<FormData>>,
    company_name_search: Signal<String>,
) -> Element {
    rsx! {
        form { class: "flex flex-col", onsubmit: create_on_submit(callback),
            input {
                name: CompanyFieldName::Name.name(),
                value: company_name_search,
                oninput: company_search,
            }
            input { r#type: "submit" }
        }
    }
}

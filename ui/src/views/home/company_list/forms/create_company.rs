use crate::helpers::log_error;
use crate::StoreType;
use dioxus::prelude::*;
use storage::prelude::{BaseStore, Company, PartialCompany};
use uuid::Uuid;

fn create_on_submit(callback: Callback<Uuid>) -> impl FnMut(FormEvent) -> () {
    move |e: FormEvent| {
        e.prevent_default();
        if let Ok(company) = e
            .parsed_values::<PartialCompany>()
            .map_err(log_error)
            .and_then(|form_data| Company::new_from_partial(form_data).map_err(log_error))
        {
            // If the company was successfully created, save it
            spawn(async move {
                let company_id = company.id;
                let mut stores = use_context::<StoreType>();
                stores.store(company).await.unwrap_or_else(log_error);
                callback(company_id);
            });
        }
    }
}

#[component]
pub fn CreateCompany(
    callback: Callback<Uuid>,
    company_search: Callback<Event<FormData>>,
    company_name_search: Signal<String>,
) -> Element {
    rsx! {
        form {
            class: "flex flex-col",
            onsubmit: create_on_submit(callback),
            input { id: "add_role", name: "name",
                    value: company_name_search,
                    oninput: company_search,}
            input { r#type: "submit" }
        }
    }
}

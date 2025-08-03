use crate::helpers::ModifyWithFormData;
use crate::{Editable, Route, StoreType};
use application_context::prelude::*;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn CompanyListItem(company: Arc<Company>, reload_companies: Callback) -> Element {
    let stores = use_context::<StoreType>();
    let context = use_context::<Signal<ApplicationContext>>();
    let form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);

    let company_id = company.id;
    let company_name = company.name.clone();

    let input_name = "company_name";

    if let Some(event) = form_receiver() {
        let mut stores = stores.clone();
        let mut company = Arc::unwrap_or_clone(company);
        if company.modify_with_form_data(&event).is_ok() && !company.name.is_empty() {
            spawn(async move {
                stores
                    .store(company)
                    .await
                    .expect("Could not store company");
                navigator().push(Route::HomeCompany { company_id });
            });
        }
    }

    let checked = context()
        .get_company()
        .map(|selected_company| selected_company.id == company_id)
        .unwrap_or(false);
    let display = rsx! {
        input {
            id: "{company_id}",
            r#type: "radio",
            name: "company",
            checked,
            onchange: move |_| {
                spawn(async move {
                    navigator().push(Route::HomeCompany { company_id });
                });
            },
        }
        label { r#for: "{company_id}", "{company_name}" }
    };

    let editable = rsx! {
        input {
            id: "{company_id}",
            r#type: "text",
            name: input_name,
            value: "{company_name}",
        }
    };

    rsx! {
        li { key: id,
            if form_receiver().is_none() {
                Editable { display, editable, form_receiver }
            } else {
                "pending"
            }
        }
    }
}

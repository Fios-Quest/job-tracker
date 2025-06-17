use crate::{Editable, Route, StoreType};
use dioxus::prelude::*;
use storage::prelude::*;

#[component]
pub fn CompanyListItem(company: Company, reload_companies: Callback) -> Element {
    let stores = use_context::<StoreType>();
    let context = use_context::<Signal<ApplicationContext>>();
    let Company { id, name, .. } = company.clone();
    let form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);

    let input_name = "company_name";

    if let Some(event) = form_receiver() {
        let mut stores = stores.clone();
        let company = company.clone();
        let company_name = event.values().get(input_name).map(|v| v.as_value());
        spawn(async move {
            if let Some(name) = company_name {
                if !name.is_empty() {
                    let company = Company { name, ..company };
                    let company_id = company.id;
                    stores
                        .store(company)
                        .await
                        .expect("Could not store company");
                    navigator().push(Route::HomeCompany { company_id });
                }
            }
        });
    }

    let company_id = company.id;
    let checked = context()
        .get_company()
        .map(|selected_company| selected_company.id == company_id)
        .unwrap_or(false);
    let display = rsx! {
        input {
            id: id.to_string(),
            r#type: "radio",
            name: "company",
            checked,
            onchange: move |_| {
                spawn(async move {
                    navigator().push(Route::HomeCompany { company_id });
                });
            },
        }
        label { r#for: id.to_string(), "{name}" }
    };

    let editable = rsx! {
        input {
            id: id.to_string(),
            r#type: "text",
            name: input_name,
            value: name,
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

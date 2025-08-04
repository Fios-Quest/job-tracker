use crate::helpers::ModifyWithFormData;
use crate::{Editable, Route, StoreType};
use application_context::prelude::*;
use dioxus::prelude::*;
use storage::prelude::*;

#[component]
pub fn CompanyListItem(company: Company, reload_companies: Callback) -> Element {
    let stores = use_context::<StoreType>();
    let context = use_context::<Signal<ApplicationContext>>();
    let form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);
    let id = company.id;

    let input_name = "name";

    let checked = context()
        .get_company()
        .map(|selected_company| selected_company.id == company.id)
        .unwrap_or(false);
    let display = rsx! {
        input {
            id: "{id}",
            r#type: "radio",
            name: "company",
            checked,
            onchange: move |_| {
                spawn(async move {
                    navigator()
                        .push(Route::HomeCompany {
                            company_id: company.id,
                        });
                });
            },
        }
        label { r#for: "{company.id}", "{company.name}" }
    };

    let editable = rsx! {
        input {
            id: "{company.id}",
            r#type: "text",
            name: input_name,
            value: "{company.name}",
        }
    };

    if let Some(event) = form_receiver() {
        let mut stores = stores.clone();
        let mut company = company;
        let result = company.modify_with_form_data(&event);
        if result.is_ok() && !company.name.is_empty() {
            spawn(async move {
                let company_id = company.id;
                stores
                    .store(company)
                    .await
                    .expect("Could not store company");
                navigator().push(Route::HomeCompany { company_id });
            });
        }
    }

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

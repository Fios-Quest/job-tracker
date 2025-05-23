use crate::{Editable, StoreContext};
use dioxus::prelude::*;
use storage::{ApplicationContext, Company, Store, Stores};

#[component]
pub fn CompanyListItem(company: Company, reload_companies: Callback) -> Element {
    let mut application_context = use_context::<Signal<ApplicationContext>>();
    let stores = use_context::<StoreContext>();
    let Company { id, name, .. } = company.clone();
    let mut form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);

    let input_name = "company_name";

    if let Some(event) = form_receiver() {
        let stores = stores.clone();
        let company = company.clone();
        let company_name = event.values().get(input_name).map(|v| v.as_value());
        spawn(async move {
            if let Some(name) = company_name {
                if !name.is_empty() {
                    let company = Company { name, ..company };
                    let _result = stores.update_company(&company).await;
                    reload_companies(());
                    form_receiver.set(None);
                }
            }
        });
    }

    let display = rsx! {
        input {
                id: id.to_string(),
                r#type: "radio",
                name: "company",
                onchange: move |_| application_context.set(application_context().set_company_id(id)),
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

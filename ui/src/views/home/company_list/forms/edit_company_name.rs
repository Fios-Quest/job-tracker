use crate::helpers::{log_error, report_if_error};
use crate::StoreType;
use dioxus::prelude::*;
use storage::prelude::{ApplyPartial, BaseStore, Company, PartialCompany};
use uuid::Uuid;

// ToDo: Names are so common, this feels like it could be made generic
fn create_on_submit(company: Company, callback: Callback<Uuid>) -> impl FnMut(FormEvent) {
    move |e: FormEvent| {
        e.prevent_default();
        if let Ok(form_data) = e.parsed_values::<PartialCompany>().map_err(log_error) {
            let mut company = company.clone();
            spawn(async move {
                company.apply(form_data);
                let id = company.id;
                let mut stores = use_context::<StoreType>();
                report_if_error!(stores.store(company).await);
                callback(id);
            });
        }
    }
}

#[component]
pub fn EditCompanyName(company: Company, callback: Callback<Uuid>) -> Element {
    let name = company.name.clone();
    let id = company.id;
    rsx! {
        form { onsubmit: create_on_submit(company, callback),
            input {
                id: "{id}",
                r#type: "text",
                name: "name",
                value: name,
            }
            input { r#type: "submit" }
        }
    }
}

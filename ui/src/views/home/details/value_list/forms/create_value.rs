use crate::helpers::log_error;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{BaseStore, Company, PartialValue, Value};

fn create_on_submit(company: Arc<Company>, callback: Callback<Value>) -> impl FnMut(FormEvent) {
    move |e: FormEvent| {
        e.prevent_default();
        if let Ok(role) = e
            .parsed_values::<PartialValue>()
            .map_err(log_error)
            .and_then(|form_data| {
                company
                    .create_value_from_partial(form_data)
                    .map_err(log_error)
            })
        {
            // If the role was successfully created, save it
            spawn(async move {
                let mut stores = use_context::<StoreType>();
                stores.store(role.clone()).await.unwrap_or_else(log_error);
                callback(role);
            });
        }
    }
}

#[component]
pub fn CreateValue(company: Arc<Company>, callback: Callback<Value>) -> Element {
    rsx! {
        form { onsubmit: create_on_submit(company, callback),
            input { name: "name", value: "" }
            textarea { name: "description", value: "" }
            input { r#type: "submit" }
        }
    }
}

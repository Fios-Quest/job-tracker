use crate::helpers::{log_error, report_if_error};
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{ApplyPartial, BaseStore, PartialValue, Value};

fn create_on_submit(value: Arc<Value>, callback: Callback<Value>) -> impl FnMut(FormEvent) {
    move |e: FormEvent| {
        e.prevent_default();
        if let Ok(form_data) = e.parsed_values::<PartialValue>().map_err(log_error) {
            let mut value = Value::clone(&value);
            spawn(async move {
                value.apply(form_data);
                let new_value = value.clone();
                let mut stores = use_context::<StoreType>();
                report_if_error!(stores.store(value).await);
                callback(new_value);
            });
        }
    }
}

#[component]
pub fn EditValue(value: Arc<Value>, callback: Callback<Value>) -> Element {
    rsx! {
        form { onsubmit: create_on_submit(value.clone(), callback),
            input { name: "name", value: "{value.name}" }
            textarea { name: "description", value: "{value.description}" }
            input { r#type: "submit" }
        }
    }
}

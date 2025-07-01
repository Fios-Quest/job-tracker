use crate::{StoreType, ValueListItem};
use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{BaseStore, Company, RecallByCompany, Value};
use storage::StorageError;
use uuid::Uuid;

const VALUE_NAME_FIELD: &str = "value_name";
const VALUE_DESCRIPTION_FIELD: &str = "value_description";

fn form_date_to_value(company_id: Uuid, form_data: &FormData) -> Option<Value> {
    let name = form_data.values().get(VALUE_NAME_FIELD)?.as_value();
    let description = form_data
        .values()
        .get(VALUE_DESCRIPTION_FIELD)
        .map(|v| v.as_value())
        .unwrap_or_default();
    if name.is_empty() {
        None
    } else {
        Some(Value::new(company_id, name, description))
    }
}

fn handle_storage_error(error: anyhow::Error) -> Option<String> {
    tracing::error!("Storage Error: {:?}", error);

    match error.downcast_ref::<StorageError>() {
        Some(StorageError::NotFound) => Some("No value found".to_string()),
        Some(StorageError::AlreadyExists) => Some("Value already exists".to_string()),
        _ => Some("A database error has occurred".to_string()),
    }
}

#[component]
pub fn ValueList(company: Arc<Company>) -> Element {
    let company_id = company.id;

    let mut error_message = use_signal(|| None);

    let mut values_resource = use_resource(move || async move {
        let result = use_context::<StoreType>()
            .recall_by_company(company_id)
            .await;
        match result {
            Ok(values) => values,
            Err(e) => {
                error_message.set(handle_storage_error(e));
                Vec::with_capacity(0)
            }
        }
    });

    let values = values_resource().unwrap_or_default();
    let values_list = values.iter().cloned().map(|value| {
        rsx! {
            ValueListItem { value }
        }
    });

    let create_value = move |event: Event<FormData>| async move {
        let value = form_date_to_value(company_id, &event);
        if let Some(value) = value {
            let mut store = use_context::<StoreType>();
            let result = store.store(value).await;
            match result {
                Ok(_) => {
                    error_message.set(None);
                    // Rerun the resource
                    values_resource.restart();
                }
                Err(e) => {
                    error_message.set(handle_storage_error(e));
                }
            }
        }
    };

    rsx! {
        div { id: "flags",
            h3 { "Values" }
            ul { {values_list} }

            form { onsubmit: create_value,
                input { name: VALUE_NAME_FIELD, value: "" }
                textarea { name: VALUE_DESCRIPTION_FIELD, value: "" }
                input { r#type: "submit" }
            }
        }
    }
}

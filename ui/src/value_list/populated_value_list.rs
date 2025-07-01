use crate::value_list::{VALUE_DESCRIPTION_FIELD, VALUE_NAME_FIELD};
use crate::{StoreType, ValueListItem};
use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{BaseStore, Company, RecallByCompany};
use storage::StorageError;

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

    let mut values_resource = use_resource(use_reactive!(|(company_id)| async move {
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
    }));

    let reload_values = use_callback(move |()| values_resource.restart());
    let values = values_resource().unwrap_or_default();
    let values_list = values.iter().cloned().map(move |value| {
        rsx! {
            ValueListItem { value, reload_values }
        }
    });

    let create_value = move |event: Event<FormData>| async move {
        let value = super::form_date_to_value(company_id, &event);
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

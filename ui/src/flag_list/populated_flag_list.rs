use super::flag_list_item::FlagListItem;
use crate::error_message::ErrorMessage;
use crate::StoreType;
use dioxus::{logger::tracing, prelude::*};
use std::str::FromStr;
use std::sync::Arc;
use storage::prelude::*;

fn handle_storage_error(error: anyhow::Error) -> Option<String> {
    tracing::error!("Storage Error: {:?}", error);

    match error.downcast_ref::<StorageError>() {
        Some(StorageError::NotFound) => Some("No flag found".to_string()),
        Some(StorageError::AlreadyExists) => Some("Flag already exists".to_string()),
        _ => Some("A database error has occurred".to_string()),
    }
}

#[component]
pub fn PopulatedFlagList(company: Arc<Company>) -> Element {
    let company_id = company.id;

    let stores = use_context::<StoreType>();
    let mut error_message = use_signal(|| None);

    // Get flags for company
    let mut flags_resource = use_resource(use_reactive!(|(company_id)| async move {
        let result = use_context::<StoreType>()
            .recall_by_company(company_id)
            .await;
        match result {
            Ok(flags) => flags,
            Err(e) => {
                error_message.set(handle_storage_error(e));
                Vec::with_capacity(0)
            }
        }
    }));
    let reload_flags = use_callback(move |()| flags_resource.restart());
    let flags = flags_resource().unwrap_or_default();
    let flags_list = flags.iter().cloned().map(|flag| {
        rsx! {
            FlagListItem { flag, reload_flags }
        }
    });

    let create_flag = move |event: Event<FormData>| {
        let mut stores = stores.clone();
        async move {
            let flag_name = event.values().get("flag_name").map(|v| v.as_value());
            let flag_color = event
                .values()
                .get("flag_color")
                .and_then(|v| FlagColor::from_str(&v.as_value()).ok());

            if let (Some(flag_name), Some(flag_color)) = (flag_name, flag_color) {
                if !flag_name.is_empty() {
                    // Store the name
                    let flag = match flag_color {
                        FlagColor::Green => Flag::new_green(company_id, flag_name),
                        FlagColor::Red => Flag::new_red(company_id, flag_name),
                    };
                    let result = stores.store(flag).await;

                    match result {
                        Ok(_) => {
                            // Reset the values to empty
                            error_message.set(None);

                            // Rerun the resource
                            flags_resource.restart();
                        }
                        Err(e) => {
                            error_message.set(handle_storage_error(e));
                        }
                    }
                }
            }
        }
    };

    rsx! {
        div { id: "flags",

            h3 { "Flags" }

            ul { {flags_list} }

            if let Some(message) = error_message() {
                ErrorMessage { message }
            }

            form { onsubmit: create_flag,
                select { id: "flag_color", name: "flag_color",
                    option { value: "red", "🚩 Red" }
                    option { value: "green", "💚 Green" }
                }
                input { id: "add_flag", name: "flag_name" }
                input { r#type: "submit" }
            }
        }
    }
}

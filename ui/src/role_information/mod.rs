use crate::error_message::ErrorMessage;
use crate::StoreContext;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use storage::{Role, SetDescription, StorageError, Store, Stores};

fn handle_storage_error(error: StorageError) -> Option<String> {
    tracing::error!("Role Storage Error: {:?}", error);

    match error {
        StorageError::NotFound => Some("No role found".to_string()),
        _ => Some("A database error has occurred".to_string()),
    }
}

#[component]
fn PopulatedRoleDescription(role: Role) -> Element {
    let stores = use_context::<StoreContext>();
    let role_description = role.description.clone();
    let mut role_description_value = use_signal(|| role_description);
    let mut error_message = use_signal(|| None);
    tracing::info!("Update {:?}", role);

    let update_role = move |event: Event<FormData>| {
        let stores = stores.clone();
        let mut role = role.clone();
        tracing::info!("Update Role");
        async move {
            let role_description = event.values().get("role_description").map(|v| v.as_value());
            tracing::info!("Event {:?}", event.values());

            if let Some(role_description) = role_description {
                if !role_description.is_empty() {
                    // Store the name
                    let mut stores_lock = stores.lock().await;

                    role.set_description(role_description.clone());
                    tracing::info!("Role set to {:?}", role);
                    let result = stores_lock.role_store().update(role).await;

                    match result {
                        Ok(_) => {
                            // Reset the values to empty
                            role_description_value.set(role_description);
                            error_message.set(None);
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
        h3 { "Role Description" }


        form { onsubmit: update_role,
            textarea { id: "role_description", name: "role_description", {role_description_value} }
            if let Some(message) = error_message() {
                ErrorMessage { message }
            }
            input { r#type: "submit" }
        }
    }
}

#[component]
pub fn RoleDescription(role: Option<Role>) -> Element {
    match role {
        Some(role) => rsx! {
            PopulatedRoleDescription { role }
        },
        None => rsx!(),
    }
}

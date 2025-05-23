use super::role_list_item::RoleListItem;
use crate::error_message::ErrorMessage;
use crate::StoreContext;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use storage::StorageError;
use storage::{Role, Timestamp};
use uuid::Uuid;

fn handle_storage_error(error: anyhow::Error) -> Option<String> {
    tracing::error!("Storage Error: {:?}", error);

    match error.downcast_ref::<StorageError>() {
        Some(StorageError::NotFound) => Some("No role found".to_string()),
        Some(StorageError::AlreadyExists) => Some("Role already exists".to_string()),
        _ => Some("A database error has occurred".to_string()),
    }
}

#[component]
pub fn PopulatedRoleList(company_id: Uuid) -> Element {
    let stores = use_context::<StoreContext>();
    let mut role_name_value = use_signal(|| "");
    let mut error_message = use_signal(|| None);

    // Get roles for company
    let mut roles_resource = use_resource(use_reactive!(|(company_id,)| async move {
        let result = use_context::<StoreContext>()
            .get_roles_for_company(company_id)
            .await;
        match result {
            Ok(roles) => roles,
            Err(e) => {
                error_message.set(handle_storage_error(e));
                Vec::with_capacity(0)
            }
        }
    }));
    let reload_roles = use_callback(move |()| roles_resource.restart());
    let roles = roles_resource().unwrap_or_default();
    let roles_list = roles.into_iter().map(|role| {
        rsx! {
            RoleListItem { role, reload_roles }
        }
    });

    let create_role = move |event: Event<FormData>| {
        let stores = stores.clone();
        async move {
            let role_name = event.values().get("role_name").map(|v| v.as_value());

            if let Some(role_name) = role_name {
                if !role_name.is_empty() {
                    // Store the name
                    let result = stores
                        .create_role(&Role::new(company_id, role_name, Timestamp::now()))
                        .await;

                    match result {
                        Ok(_) => {
                            // Reset the values to empty
                            role_name_value.set("");
                            error_message.set(None);

                            // Rerun the resource
                            roles_resource.restart();
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
        div { id: "roles", class: "{company_id}",

            h3 { "Roles" }

            ul { {roles_list} }

            if let Some(message) = error_message() {
                ErrorMessage { message }
            }

            form { onsubmit: create_role,
                input {
                    id: "add_role",
                    name: "role_name",
                    value: role_name_value,
                }
                input { r#type: "submit" }
            }
        }
    }
}

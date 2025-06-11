use super::role_list_item::RoleListItem;
use crate::Route::HomeRole;
use crate::StoreType;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

fn handle_storage_error(error: anyhow::Error) -> Option<String> {
    tracing::error!("Storage Error: {:?}", error);

    match error.downcast_ref::<StorageError>() {
        Some(StorageError::NotFound) => Some("No role found".to_string()),
        Some(StorageError::AlreadyExists) => Some("Role already exists".to_string()),
        _ => Some("A database error has occurred".to_string()),
    }
}

#[component]
pub fn PopulatedRoleList(company: Arc<Company>) -> Element {
    let stores = use_context::<StoreType>();
    let role_name_value = use_signal(|| "");
    let mut error_message = use_signal(|| None);

    // Get roles for company
    let mut roles_resource = use_resource(use_reactive!(|(company)| async move {
        let result = use_context::<StoreType>()
            .recall_by_company(company.id)
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
        let mut stores = stores.clone();
        let cloned_company = company.clone();
        async move {
            let role_name = event.values().get("role_name").map(|v| v.as_value());

            if let Some(role_name) = role_name {
                if !role_name.is_empty() {
                    // Create the role (and remember its ids)
                    let new_role = cloned_company.create_role(role_name, Timestamp::now());
                    let company_id = new_role.company_id;
                    let role_id = new_role.id;

                    // Store the name
                    stores
                        .store(new_role)
                        .await
                        .expect("Could not store company");

                    // Navigate away from the page
                    navigator().push(HomeRole {
                        company_id,
                        role_id,
                    });
                }
            }
        }
    };

    rsx! {
        div { id: "roles",

            h3 { class: "text-2xl", "Roles" }

            ul { {roles_list} }

            form { class: "flex flex-col gap-y-2", onsubmit: create_role,
                input {
                    id: "add_role",
                    class: "bg-slate-800 text-slate-200 rounded",
                    name: "role_name",
                    value: role_name_value,
                }
                input {
                    class: "bg-slate-400 text-slate-900 rounded cursor-pointer m-auto py-1.5 px-4",
                    r#type: "submit",
                }
            }
        }
    }
}

mod role_list_item;

use crate::roles_list::role_list_item::RoleListItem;
use crate::StoreContext;
use dioxus::prelude::*;
use std::sync::{Arc, Mutex};
use storage::{Role, RoleStore, Store, StubRoleStore, Timestamp};
use uuid::Uuid;

#[component]
pub fn EmptyRolesList() -> Element {
    rsx! {
        div { id: "roles" }

        h3 { "Roles" }

        p { "Select a company to see Roles" }
    }
}

#[component]
pub fn RolesList(company_id: Uuid) -> Element {
    let stores = use_context::<Arc<Mutex<StoreContext>>>();
    let mut role_name_value = use_signal(|| "");

    // Get roles for company
    let mut roles_resource = use_resource(use_reactive!(|(company_id,)| async move {
        use_context::<Arc<Mutex<StoreContext>>>()
            .lock()
            .expect("Could not lock role store")
            .role_store()
            .get_for_company(company_id)
            .await
            .expect("Did not get roles")
    }));
    let roles = roles_resource().unwrap_or_default();
    let roles_list = roles.iter().cloned().map(|role| {
        rsx! {
            RoleListItem { role }
        }
    });

    let create_role = move |event: Event<FormData>| {
        let stores = stores.clone();
        async move {
            let role_name = event.values().get("role_name").map(|v| v.as_value());

            if let Some(role_name) = role_name {
                if !role_name.is_empty() {
                    // Store the name
                    let mut stores_lock = stores.lock().expect("Could not lock role store");
                    stores_lock
                        .role_store()
                        .create(Role::new(company_id, role_name, Timestamp::now()))
                        .await
                        .expect("Could not store new role");

                    // Reset the values to empty
                    role_name_value.set("");

                    // Rerun the resource
                    roles_resource.restart();
                }
            }
        }
    };

    rsx! {
        div {
            id: "roles",
            class: "{company_id}",

            h3 { "Roles" }

            ul {
                { roles_list }
            }

            form {
                onsubmit: create_role,
                input { id: "add_role", name: "role_name", value: role_name_value }
                input { r#type: "submit" }
            }
        }
    }
}

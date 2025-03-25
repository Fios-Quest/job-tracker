mod role_list_item;

use crate::roles_list::role_list_item::RoleListItem;
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
    let role_store = use_context::<Arc<Mutex<StubRoleStore>>>();
    let mut role_name_value = use_signal(|| "");

    // Get roles for company
    let mut roles_resource = use_resource(use_reactive!(|(company_id,)| async move {
        use_context::<Arc<Mutex<StubRoleStore>>>()
            .lock()
            .expect("Could not lock role store")
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
        let role_store = role_store.clone();
        async move {
            let role_name = event.values().get("role_name").map(|v| v.as_value());

            if let Some(role_name) = role_name {
                if !role_name.is_empty() {
                    // Store the name
                    role_store
                        .lock()
                        .expect("Could not lock role store")
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

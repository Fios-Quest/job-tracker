use crate::helpers::{log_error, report_if_error};
use crate::StoreType;
use dioxus::prelude::*;
use serde::Deserialize;
use storage::prelude::{BaseStore, Role};

#[derive(Deserialize)]
struct EditRoleNameData {
    name: String,
}

fn create_on_submit(role: Role, callback: Callback) -> impl FnMut(FormEvent) -> () {
    move |e: FormEvent| {
        e.prevent_default();
        if let Ok(form_data) = e.parsed_values::<EditRoleNameData>().map_err(log_error) {
            let mut role = Role::clone(&role);
            spawn(async move {
                role.name = form_data.name;
                let mut stores = use_context::<StoreType>();
                report_if_error!(stores.store(role).await);
                callback(());
            });
        }
    }
}

#[component]
pub fn EditRoleName(role: Role, callback: Callback) -> Element {
    let name = role.name.clone();
    rsx! {
        form {
            onsubmit: create_on_submit(role, callback),
            input {
                r#type: "text",
                name: "name",
                value: name,
            }
            input {
                r#type: "submit",
            }
        }
    }
}

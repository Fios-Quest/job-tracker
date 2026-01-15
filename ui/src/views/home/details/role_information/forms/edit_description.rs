use crate::helpers::{log_error, report_if_error};
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{ApplyPartial, BaseStore, PartialRole, Role, RoleFieldName};

fn create_on_submit(role: Arc<Role>, callback: Callback<Role>) -> impl FnMut(FormEvent) {
    move |e: FormEvent| {
        e.prevent_default();
        if let Ok(form_data) = e.parsed_values::<PartialRole>().map_err(log_error) {
            let mut role = Role::clone(&role);
            spawn(async move {
                role.apply(form_data);
                let mut stores = use_context::<StoreType>();
                report_if_error!(stores.store(role.clone()).await);
                callback(role);
            });
        }
    }
}

#[component]
pub fn EditRoleDescription(role: Arc<Role>, callback: Callback<Role>) -> Element {
    rsx! {
        form { onsubmit: create_on_submit(role.clone(), callback),
            textarea { name: RoleFieldName::Description.name(), "{role.description}" }
        }
    }
}

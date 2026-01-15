use crate::helpers::{log_error, report_if_error};
use crate::StoreType;
use dioxus::prelude::*;
use storage::prelude::{ApplyPartial, BaseStore, PartialRole, Role};

fn create_on_submit(role: Role, callback: Callback) -> impl FnMut(FormEvent) {
    move |e: FormEvent| {
        e.prevent_default();
        if let Ok(form_data) = e.parsed_values::<PartialRole>().map_err(log_error) {
            let mut role = Role::clone(&role);
            spawn(async move {
                role.apply(form_data);
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
        form { onsubmit: create_on_submit(role, callback),
            input { r#type: "text", name: "name", value: name }
            input { r#type: "submit" }
        }
    }
}

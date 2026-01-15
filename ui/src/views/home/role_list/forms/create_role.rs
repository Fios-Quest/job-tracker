use crate::helpers::log_error;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{BaseStore, Company, PartialRole};
use uuid::Uuid;

fn create_on_submit(company: Arc<Company>, callback: Callback<Uuid>) -> impl FnMut(FormEvent) {
    move |e: FormEvent| {
        e.prevent_default();
        // Get the partial from the form data, then create the role from the partial
        if let Ok(role) = e
            .parsed_values::<PartialRole>()
            .map_err(log_error)
            .and_then(|form_data| {
                company
                    .create_role_from_partial(form_data)
                    .map_err(log_error)
            })
        {
            // If the role was successfully created, save it
            spawn(async move {
                let role_id = role.id;
                let mut stores = use_context::<StoreType>();
                stores.store(role).await.unwrap_or_else(log_error);
                callback(role_id);
            });
        }
    }
}

#[component]
pub fn CreateRole(company: Arc<Company>, callback: Callback<Uuid>) -> Element {
    rsx! {
        form {
            class: "flex flex-col",
            onsubmit: create_on_submit(company, callback),
            input { id: "add_role", name: "name", value: "" }
            input { r#type: "submit" }
        }
    }
}

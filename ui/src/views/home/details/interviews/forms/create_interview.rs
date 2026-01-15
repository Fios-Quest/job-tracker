use crate::helpers::log_error;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{BaseStore, PartialInterview, Role};
use uuid::Uuid;

fn create_on_submit(role: Arc<Role>, callback: Callback<Uuid>) -> impl FnMut(FormEvent) {
    move |e: FormEvent| {
        e.prevent_default();
        if let Ok(interview) = e
            .parsed_values::<PartialInterview>()
            .map_err(log_error)
            .and_then(|form_data| {
                role.create_interview_from_partial(form_data)
                    .map_err(log_error)
            })
        {
            // If the role was successfully created, save it
            spawn(async move {
                let id = interview.id;
                let mut stores = use_context::<StoreType>();
                stores.store(interview).await.unwrap_or_else(log_error);
                callback(id);
            });
        }
    }
}

#[component]
pub fn CreateInterview(role: Arc<Role>, callback: Callback<Uuid>) -> Element {
    let mut is_editable = use_signal(|| false);
    if is_editable() {
        rsx! {
            form { onsubmit: create_on_submit(role, callback),
                input { name: "name" }
                button { "✅" }
            }
            a { href: "#", onclick: move |_| is_editable.set(false), "❌" }
        }
    } else {
        rsx! {
            a { href: "#", onclick: move |_| is_editable.set(true), "+ Interview" }
        }
    }
}

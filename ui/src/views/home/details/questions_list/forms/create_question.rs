use crate::helpers::log_error;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{BaseStore, PartialQuestion, Role};
use uuid::Uuid;

fn create_on_submit(role: Arc<Role>, callback: Callback<Uuid>) -> impl FnMut(FormEvent) -> () {
    move |e: FormEvent| {
        e.prevent_default();
        if let Ok(question) = e
            .parsed_values::<PartialQuestion>()
            .map_err(log_error)
            .and_then(|form_data| {
                role.create_question_from_partial(form_data)
                    .map_err(log_error)
            })
        {
            // If the role was successfully created, save it
            spawn(async move {
                let id = question.id;
                let mut stores = use_context::<StoreType>();
                stores.store(question).await.unwrap_or_else(log_error);
                callback(id);
            });
        }
    }
}

#[component]
pub fn CreateQuestion(role: Arc<Role>, callback: Callback<Uuid>) -> Element {
    rsx! {
        form {
            onsubmit: create_on_submit(role, callback),
            input { name: "question", value: "" }
            textarea { name: "answer", value: "" }
            input { r#type: "submit" }
        }
    }
}

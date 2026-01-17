use crate::helpers::log_error;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{BaseStore, PartialQuestion, Question, QuestionFieldName, Role};

fn create_on_submit(role: Arc<Role>, callback: Callback<Question>) -> impl FnMut(FormEvent) {
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
                let mut stores = use_context::<StoreType>();
                stores
                    .store(question.clone())
                    .await
                    .unwrap_or_else(log_error);
                callback(question);
            });
        }
    }
}

#[component]
pub fn CreateQuestion(role: Arc<Role>, callback: Callback<Question>) -> Element {
    rsx! {
        form { onsubmit: create_on_submit(role, callback),
            input { name: QuestionFieldName::Name.name(), value: "" }
            textarea { name: QuestionFieldName::Answer.name(), value: "" }
            input { r#type: "submit" }
        }
    }
}

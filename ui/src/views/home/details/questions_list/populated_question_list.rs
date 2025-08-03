use super::{QUESTION_ANSWER_FIELD, QUESTION_NAME_FIELD};
use crate::helpers::CreatePartialFromFormData;
use crate::questions_list::QuestionListItem;
use crate::StoreType;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;
use storage::StorageError;

fn handle_storage_error(error: anyhow::Error) -> Option<String> {
    tracing::error!("Storage Error: {:?}", error);

    match error.downcast_ref::<StorageError>() {
        Some(StorageError::NotFound) => Some("No question found".to_string()),
        Some(StorageError::AlreadyExists) => Some("Question already exists".to_string()),
        _ => Some("A database error has occurred".to_string()),
    }
}

#[component]
pub fn QuestionList(role: Arc<Role>) -> Element {
    let role_id = role.id;

    let mut error_message = use_signal(|| None);

    let mut questions_resource = use_resource(use_reactive!(|(role_id)| async move {
        let result = use_context::<StoreType>().recall_by_role(role_id).await;
        match result {
            Ok(questions) => questions.into_iter().map(Arc::new).collect(),
            Err(e) => {
                error_message.set(handle_storage_error(e));
                Vec::with_capacity(0)
            }
        }
    }));

    let reload_questions = use_callback(move |()| questions_resource.restart());
    let questions = questions_resource().unwrap_or_default();
    let questions_list = questions.iter().cloned().map(move |question| {
        rsx! {
            QuestionListItem { question, reload_questions }
        }
    });

    let create_question = move |event: Event<FormData>| {
        let role = role.clone();
        let partial_question = PartialQuestion::from_form_data(&event)
            .expect("Could not parse form data for question");
        let question = Question::new_from_partial(role.id, partial_question)
            .expect("Could not create question from partial");
        async move {
            let mut store = use_context::<StoreType>();
            let result = store.store(question).await;
            match result {
                Ok(_) => {
                    error_message.set(None);
                    // Rerun the resource
                    questions_resource.restart();
                }
                Err(e) => {
                    error_message.set(handle_storage_error(e));
                }
            }
        }
    };

    rsx! {
        div { id: "flags",
            h3 { "Questions" }
            ul { {questions_list} }

            form { onsubmit: create_question,
                input { name: QUESTION_NAME_FIELD, value: "" }
                textarea { name: QUESTION_ANSWER_FIELD, value: "" }
                input { r#type: "submit" }
            }
        }
    }
}

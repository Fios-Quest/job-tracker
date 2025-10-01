use super::{QUESTION_ANSWER_FIELD, QUESTION_NAME_FIELD};
use crate::helpers::{unwrap_or_report_and_return, CreatePartialFromFormData};
use crate::questions_list::QuestionListItem;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn QuestionList(role: Arc<Role>) -> Element {
    let role_id = role.id;

    let mut questions_resource = use_resource(use_reactive!(|(role_id)| async move {
        let questions =
            unwrap_or_report_and_return!(use_context::<StoreType>().recall_by_role(role_id).await);

        questions.into_iter().map(Arc::new).collect::<Vec<_>>()
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
        async move {
            let partial_question =
                unwrap_or_report_and_return!(PartialQuestion::from_form_data(&event));
            let question =
                unwrap_or_report_and_return!(Question::new_from_partial(role.id, partial_question));
            unwrap_or_report_and_return!(use_context::<StoreType>().store(question).await);
            questions_resource.restart();
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

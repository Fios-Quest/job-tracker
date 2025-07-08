use super::{edit_question_from_form_data, QUESTION_ANSWER_FIELD, QUESTION_NAME_FIELD};
use crate::{Editable, StoreType};
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn QuestionListItem(question: Arc<Question>, reload_questions: Callback) -> Element {
    let question_id = question.id;
    let mut form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);

    if let Some(event) = form_receiver() {
        let question = edit_question_from_form_data(question.clone(), &event);
        if let Some(mut question) = question {
            question.id = question_id;
            spawn(async move {
                let mut store = use_context::<StoreType>();
                let _result = store.store(question).await;
                reload_questions(());
                form_receiver.set(None);
            });
        } else {
            form_receiver.set(None);
        }
    }

    let display = rsx! {
        header { "{question.name}" }
        "{question.answer}"
    };

    let editable = rsx! {
        input { name: QUESTION_NAME_FIELD, value: "{question.name}" }
        textarea { name: QUESTION_ANSWER_FIELD, value: "{question.answer}" }
        input { r#type: "submit" }
    };

    rsx! {
        li { id: "question-{question.id}",
            if form_receiver().is_none() {
                Editable { display, editable, form_receiver }
            } else {
                "pending"
            }
        }
    }
}

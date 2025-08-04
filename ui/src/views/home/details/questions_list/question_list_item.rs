use super::{QUESTION_ANSWER_FIELD, QUESTION_NAME_FIELD};
use crate::helpers::ModifyWithFormData;
use crate::{Editable, StoreType};
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn QuestionListItem(question: Arc<Question>, reload_questions: Callback) -> Element {
    let id = question.id;
    let mut form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);

    let display = rsx! {
        header { "{question.name}" }
        "{question.answer}"
    };

    let editable = rsx! {
        input { name: QUESTION_NAME_FIELD, value: "{question.name}" }
        textarea { name: QUESTION_ANSWER_FIELD, value: "{question.answer}" }
        input { r#type: "submit" }
    };

    if let Some(event) = form_receiver() {
        let mut question = Arc::unwrap_or_clone(question);
        question
            .modify_with_form_data(&event)
            .expect("Could not modify question with form data");
        spawn(async move {
            let mut store = use_context::<StoreType>();
            let _result = store.store(question).await;
            reload_questions(());
            form_receiver.set(None);
        });
    }

    rsx! {
        li { id: "question-{id}",
            if form_receiver().is_none() {
                Editable { display, editable, form_receiver }
            } else {
                "pending"
            }
        }
    }
}

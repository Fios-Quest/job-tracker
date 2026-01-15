use crate::questions_list::forms::EditQuestion;
use crate::Editable;
use dioxus::prelude::*;
use storage::prelude::*;

#[component]
pub fn QuestionListItem(question: Question, reload_questions: Callback) -> Element {
    let is_editable = use_signal(|| false);

    let id = question.id;

    let display = rsx! {
        header { "{question.name}" }
        "{question.answer}"
    };

    let callback = use_callback(move |_id| reload_questions(()));

    let editable = rsx! {
        EditQuestion { question, callback }
    };

    rsx! {
        li { id: "question-{id}",
            Editable { display, editable, is_editable }
        }
    }
}

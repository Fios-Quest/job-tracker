use crate::helpers::edit_with_form;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{Question, QuestionFieldName};

#[component]
pub fn EditQuestion(question: Arc<Question>, callback: Callback<Question>) -> Element {
    rsx! {
        form { onsubmit: edit_with_form(use_context::<StoreType>(), question.clone(), callback),
            input {
                name: QuestionFieldName::Name.name(),
                value: "{question.name}",
            }
            textarea {
                name: QuestionFieldName::Answer.name(),
                value: "{question.answer}",
            }
            input { r#type: "submit" }
        }
    }
}

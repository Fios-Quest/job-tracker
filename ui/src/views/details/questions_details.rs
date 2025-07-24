use crate::QuestionList;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn QuestionsDetails(role: Arc<Role>) -> Element {
    rsx! {
        h3 { "Questions" }
        QuestionList { role }
    }
}

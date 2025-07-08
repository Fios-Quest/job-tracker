use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::Interview;

#[component]
pub fn InterviewDetails(interview: Option<Arc<Interview>>) -> Element {
    rsx! {
        h3 { "Interviews" }
    }
}

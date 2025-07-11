use crate::InterviewNav;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn InterviewDetails(role: Arc<Role>) -> Element {
    // let context = use_context::<Signal<ApplicationContext>>();
    // let interview = context().get_interview();

    rsx! {
        h3 { "Interviews" }
        InterviewNav { role }
    }
}

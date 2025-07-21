use crate::{Editable, InterviewNav, StoreType};
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
fn InterviewDetailsDisplay(interview: Arc<Interview>) -> Element {
    let hosts = interview.host.join(" - ");
    let when = interview
        .date_time
        .map(|t| t.to_string())
        .unwrap_or_default();
    rsx! {
        dl { class: "interview-details",
            dt { "When:" }
            dd { "{when}" }
            dt { "Who:" }
            dd { "{hosts}" }
            dt { "Interview:" }
            dd { "{interview.name}" }
        }
        section { "{interview.notes}" }
    }
}

#[component]
fn InterviewDetailsEditable(interview: Arc<Interview>) -> Element {
    let hosts = interview.host.join("; ");
    let when = interview
        .date_time
        .map(|t| t.to_string())
        .unwrap_or_default();
    rsx! {
        dl { class: "interview-details",
            dt { "When:" }
            dd {
                input { name: "date_time", value: "{when}" }
            }
            dt { "Who:" }
            dd {
                input { name: "hosts", value: "{hosts}" }
            }
            dt { "Interview:" }
            dd {
                input { name: "name", value: "{interview.name}" }
            }
        }
        section {
            textarea { name: "notes", value: "{interview.notes}" }
        }
    }
}

#[component]
pub fn InterviewDetails(role: Arc<Role>) -> Element {
    let context = use_context::<Signal<ApplicationContext>>();
    let mut stores = use_context::<StoreType>();
    let interview = context().get_interview();
    dbg!(&interview);

    let Some(interview) = interview else {
        return rsx! {
            h3 { "Interviews" }
            InterviewNav { role }
            "No interview selected"
        };
    };

    let form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);
    if let Some(partial_interview) =
        form_receiver().and_then(|e| e.parsed_values::<PartialInterview>().ok())
    {
        let mut interview = Arc::unwrap_or_clone(interview.clone());
        interview.apply(partial_interview);
        spawn(async move { stores.store(interview).await.expect("Argh") });
    }

    let display = rsx! {
        InterviewDetailsDisplay { interview: interview.clone() }
    };
    let editable = rsx! {
        InterviewDetailsEditable { interview }
    };

    rsx! {
        h3 { "Interviews" }
        InterviewNav { role }

        div {

            Editable { display, editable, form_receiver }
        }
    }
}

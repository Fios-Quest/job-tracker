use super::interviews::InterviewNav;
use crate::helpers::unwrap_or_report_and_return;
use crate::interviews::forms::EditInterview;
use crate::{Editable, StoreType};
use application_context::prelude::*;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
fn InterviewDetailsDisplay(interview: Arc<Interview>) -> Element {
    let when = interview
        .date_time
        .map(|t| t.to_string())
        .unwrap_or_default();
    rsx! {
        dl { class: "interview-details",
            dt { "When:" }
            dd { "{when}" }
            dt { "Who:" }
            dd { "{interview.host}" }
            dt { "Interview:" }
            dd { "{interview.name}" }
        }
        section { "{interview.notes}" }
    }
}

#[component]
pub fn InterviewDetails(role: Arc<Role>) -> Element {
    let mut is_editable = use_signal(|| false);
    let context = use_context::<Signal<ApplicationContext>>();
    let interview = context().get_interview();
    let store = use_context::<StoreType>();
    let resource_store = store.clone();

    // Forcibly load a new version of the interview from the DB
    let mut interview_resource: Resource<Option<Arc<Interview>>> =
        use_resource(use_reactive!(|interview| {
            let interview = interview.clone();
            let store = resource_store.clone();
            async move {
                if let Some(interview) = interview {
                    let id = interview.id;
                    Some(unwrap_or_report_and_return!(store
                        .recall_by_id(id)
                        .await
                        .map(Arc::new)))
                } else {
                    None
                }
            }
        }));

    let interview = interview_resource().unwrap_or_default();

    let Some(interview) = interview else {
        return rsx! {
            h3 { "Interviews" }
            InterviewNav { role }
            "No interview selected"
        };
    };

    let callback = use_callback(move |_interview| {
        interview_resource.restart();
        is_editable.set(false);
    });

    let display = rsx! {
        InterviewDetailsDisplay { interview: interview.clone() }
    };
    let editable = rsx! {
        EditInterview { interview, callback }
    };

    rsx! {
        h3 { "Interviews" }
        InterviewNav { role }

        div {
            Editable { display, editable, is_editable }
        }
    }
}

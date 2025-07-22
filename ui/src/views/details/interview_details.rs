use crate::form_data_helper::ModifyWithFormData;
use crate::{Editable, InterviewNav, StoreType};
use dioxus::prelude::*;
use log::warn;
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
fn InterviewDetailsEditable(interview: Arc<Interview>) -> Element {
    let when = interview
        .date_time
        .map(|t| t.to_string())
        .unwrap_or_default();
    rsx! {
        dl { class: "interview-details",
            dt { "When:" }
            dd {
                input {
                    name: "date_time",
                    r#type: "datetime-local",
                    value: "{when}",
                }
            }
            dt { "Who:" }
            dd {
                input { name: "hosts", value: "{interview.host}" }
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

    let Some(interview) = interview else {
        return rsx! {
            h3 { "Interviews" }
            InterviewNav { role }
            "No interview selected"
        };
    };

    let form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);
    if let Some(form_data) = form_receiver() {
        let mut interview = Arc::unwrap_or_clone(interview.clone());

        match interview.modify_with_form_data(&form_data) {
            Ok(()) => {
                dbg!(&interview);
                spawn(async move {
                    let _ = stores
                        .store(interview.clone())
                        .await
                        .inspect_err(|e| warn!("{e}"));
                    let _ = context()
                        .set_interview(interview)
                        .inspect_err(|e| warn!("{e}"));
                });
            }
            Err(e) => {
                dbg!(&e);
                warn!("{e}");
            }
        }
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

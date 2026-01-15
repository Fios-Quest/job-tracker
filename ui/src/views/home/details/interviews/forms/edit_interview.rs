use crate::helpers::{log_error, report_if_error};
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{ApplyPartial, BaseStore, Interview, InterviewFieldName, PartialInterview};

fn create_on_submit(
    interview: Arc<Interview>,
    callback: Callback<Interview>,
) -> impl FnMut(FormEvent) {
    move |e: FormEvent| {
        e.prevent_default();
        if let Ok(form_data) = e.parsed_values::<PartialInterview>().map_err(log_error) {
            let mut interview = Interview::clone(&interview);
            spawn(async move {
                interview.apply(form_data);
                let mut stores = use_context::<StoreType>();
                report_if_error!(stores.store(interview.clone()).await);
                callback(interview);
            });
        }
    }
}

#[component]
pub fn EditInterview(interview: Arc<Interview>, callback: Callback<Interview>) -> Element {
    let when = interview
        .date_time
        .map(|t| t.to_string())
        .unwrap_or_default();
    rsx! {
        form { onsubmit: create_on_submit(interview.clone(), callback),
            dl { class: "interview-details",
                dt { "When:" }
                dd {
                    input {
                        name: InterviewFieldName::DateTime.name(),
                        r#type: "datetime-local",
                        value: "{when}",
                    }
                }
                dt { "Who:" }
                dd {
                    input {
                        name: InterviewFieldName::Host.name(),
                        value: "{interview.host}",
                    }
                }
                dt { "Interview:" }
                dd {
                    input {
                        name: InterviewFieldName::Name.name(),
                        value: "{interview.name}",
                    }
                }
            }
            section {
                textarea {
                    name: InterviewFieldName::Notes.name(),
                    value: "{interview.notes}",
                }
            }
        }
    }
}

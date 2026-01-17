use crate::helpers::edit_with_form;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{Interview, InterviewFieldName};

#[component]
pub fn EditInterview(interview: Arc<Interview>, callback: Callback<Interview>) -> Element {
    let when = interview
        .date_time
        .map(|t| t.to_string())
        .unwrap_or_default();
    rsx! {
        form { onsubmit: edit_with_form(use_context::<StoreType>(), interview.clone(), callback),
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
            input { r#type: "submit" }
        }
    }
}

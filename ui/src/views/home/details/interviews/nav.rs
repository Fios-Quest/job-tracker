use crate::helpers::{create_route, unwrap_or_report_and_return, CreatePartialFromFormData};
use crate::{DetailsView, StoreType};
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;
use uuid::Uuid;

const INTERVIEW_NAME_FIELD: &str = "name";

#[component]
pub fn NewInterviewForm(role: Arc<Role>, reload_interviews: Callback) -> Element {
    let mut is_editable = use_signal(|| false);

    let create_interview = move |event: Event<FormData>| {
        let role = role.clone();
        async move {
            let interview = unwrap_or_report_and_return!(PartialInterview::from_form_data(&event));
            let interview =
                unwrap_or_report_and_return!(role.create_interview_from_partial(interview));
            let mut store = use_context::<StoreType>();
            unwrap_or_report_and_return!(store.store(interview).await);
            reload_interviews(());
            is_editable.set(false);
        }
    };

    rsx! {
        if is_editable() {
            form { onsubmit: create_interview,
                input { name: INTERVIEW_NAME_FIELD }
                button { "✅" }
            }
            a { href: "#", onclick: move |_| is_editable.set(false), "❌" }
        } else {
            a { href: "#", onclick: move |_| is_editable.set(true), "+ Interview" }
        }
    }
}

#[component]
pub fn InterviewNav(role: Arc<Role>) -> Element {
    let mut interview_resource = use_resource(use_reactive!(|role| async move {
        use_context::<StoreType>()
            .recall_by_role(role.id)
            .await
            .unwrap_or_default()
    }));
    let interviews: Vec<Interview> = interview_resource().unwrap_or_default();
    let reload_interviews = use_callback(move |()| interview_resource.restart());

    let company_id = role.company_id;
    let role_id = role.id;
    let route_creator = move |interview_id: Uuid| {
        create_route(
            Some(company_id),
            Some(role_id),
            Some(interview_id),
            Some(DetailsView::Interview),
        )
    };

    rsx! {
        nav {
            ul {
                for interview in interviews {
                    li {
                        a {
                            onclick: move |_| {
                                navigator().push(route_creator(interview.id));
                            },
                            "{interview.name}"
                        }
                    }
                }

                li {
                    NewInterviewForm { role, reload_interviews }
                }
            }
        }
    }
}

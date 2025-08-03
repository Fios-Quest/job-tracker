use crate::helpers::create_route;
use crate::{DetailsView, StoreType};
use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;
use uuid::Uuid;

const INTERVIEW_NAME_FIELD: &str = "interview_name";

fn form_data_name(form_data: &FormData) -> Option<String> {
    form_data
        .values()
        .get(INTERVIEW_NAME_FIELD)
        .map(|fv| fv.as_value())
}

fn create_interview_from_form_data(role: Arc<Role>, form_data: &FormData) -> Option<Interview> {
    let name = form_data_name(form_data);
    name.map(|name| role.create_interview(name))
}

fn handle_storage_error(error: anyhow::Error) -> Option<String> {
    tracing::error!("Storage Error: {:?}", error);

    match error.downcast_ref::<StorageError>() {
        Some(StorageError::NotFound) => Some("No interview found".to_string()),
        Some(StorageError::AlreadyExists) => Some("Interview already exists".to_string()),
        _ => Some("A database error has occurred".to_string()),
    }
}

#[component]
pub fn NewInterviewForm(role: Arc<Role>, reload_interviews: Callback) -> Element {
    let mut error_message = use_signal(|| None);

    let mut is_editable = use_signal(|| false);

    let create_interview = move |event: Event<FormData>| {
        let role = role.clone();
        async move {
            let value = create_interview_from_form_data(role, &event);
            if let Some(value) = value {
                let mut store = use_context::<StoreType>();
                let result = store.store(value).await;
                match result {
                    Ok(_) => {
                        error_message.set(None);
                        reload_interviews(());
                        is_editable.set(false);
                    }
                    Err(e) => {
                        error_message.set(handle_storage_error(e));
                    }
                }
            }
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

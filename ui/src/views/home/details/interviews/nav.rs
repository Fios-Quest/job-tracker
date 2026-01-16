use crate::helpers::create_route;
use crate::interviews::forms::CreateInterview;
use crate::{DetailsView, StoreType};
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;
use uuid::Uuid;

#[component]
pub fn InterviewNav(role: Arc<Role>) -> Element {
    let mut interview_resource = use_resource(use_reactive!(|role| async move {
        use_context::<StoreType>()
            .recall_by_role(role.id)
            .await
            .unwrap_or_default()
    }));
    let interviews: Vec<Interview> = interview_resource().unwrap_or_default();
    let callback = use_callback(move |_interview| interview_resource.restart());

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
                    CreateInterview { role, callback }
                }
            }
        }
    }
}

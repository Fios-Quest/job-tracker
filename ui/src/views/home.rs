use crate::router::DetailsView;
use crate::{CompanyList, Details, RoleList, StoreType, SHOW_MODIFIERS};
use dioxus::prelude::*;
use storage::prelude::RecallById;
use storage::ApplicationContext;
use uuid::Uuid;

#[component]
pub fn Home(
    company_id: Option<Uuid>,
    role_id: Option<Uuid>,
    interview_id: Option<Uuid>,
    view: Option<DetailsView>,
) -> Element {
    let show_modifiers = if SHOW_MODIFIERS() {
        "show_modifiers"
    } else {
        ""
    };

    let store = use_context::<StoreType>();
    let mut context = use_context::<Signal<ApplicationContext>>();
    let _resource = use_resource(use_reactive!(|(company_id, role_id, interview_id)| {
        let store = store.clone();

        async move {
            if let Some(company_id) = company_id {
                if context.peek().get_company().map(|c| c.id) != Some(company_id) {
                    let company = store
                        .recall_by_id(company_id)
                        .await
                        .expect("Could not set company");
                    let new_context = context.peek().clone().set_company(company);
                    context.set(new_context);
                }
            }
            // Role _must_ be set after company
            if let Some(role_id) = role_id {
                if context.peek().get_role().map(|c| c.id) != Some(role_id) {
                    let role = store
                        .recall_by_id(role_id)
                        .await
                        .expect("Could not set role");
                    let new_context = context
                        .peek()
                        .clone()
                        .set_role(role)
                        .expect("Couldn't set role");
                    context.set(new_context);
                }
            }
            // Interview _must_ be set after role
            if let Some(interview_id) = interview_id {
                if context.peek().get_interview().map(|c| c.id) != Some(interview_id) {
                    let role = store
                        .recall_by_id(interview_id)
                        .await
                        .expect("Could not set role");
                    let new_context = context
                        .peek()
                        .clone()
                        .set_interview(role)
                        .expect("Couldn't set role");
                    context.set(new_context);
                }
            }
        }
    }));

    rsx! {
        div { id: "home", class: "flex {show_modifiers}",

            section { class: "mr-4 mt-0",
                CompanyList {}
                RoleList {}
            }

            Details { view }
        }
    }
}

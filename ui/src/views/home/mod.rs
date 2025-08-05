use crate::helpers::unwrap_or_report_and_return;
use crate::router::DetailsView;
use crate::views::home::company_list::CompanyList;
use crate::views::home::role_list::RoleList;
use crate::{Details, StoreType, SHOW_MODIFIERS};
use application_context::prelude::*;
use dioxus::prelude::*;
use storage::prelude::RecallById;
use uuid::Uuid;

pub mod company_list;
pub mod details;
pub mod role_list;

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
                    let company =
                        unwrap_or_report_and_return!(store.recall_by_id(company_id).await);
                    let new_context = context.peek().clone().set_company(company);
                    context.set(new_context);
                }
            }
            // Role _must_ be set after company
            if let Some(role_id) = role_id {
                if context.peek().get_role().map(|c| c.id) != Some(role_id) {
                    let role = unwrap_or_report_and_return!(store.recall_by_id(role_id).await);
                    let new_context =
                        unwrap_or_report_and_return!(context.peek().clone().set_role(role));
                    context.set(new_context);
                }
            }
            // Interview _must_ be set after the role
            if let Some(interview_id) = interview_id {
                if context.peek().get_interview().map(|c| c.id) != Some(interview_id) {
                    let role = unwrap_or_report_and_return!(store.recall_by_id(interview_id).await);
                    let new_context =
                        unwrap_or_report_and_return!(context.peek().clone().set_interview(role));
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

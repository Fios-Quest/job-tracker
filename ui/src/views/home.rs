use crate::{CompanyList, Details, Navbar, RoleDescription, RoleList, StoreType};
use dioxus::prelude::*;
use storage::prelude::RecallById;
use storage::ApplicationContext;
use uuid::Uuid;

#[component]
pub fn Home(
    company_id: Option<Uuid>,
    role_id: Option<Uuid>,
    interview_id: Option<Uuid>,
) -> Element {
    let store = use_context::<StoreType>();
    let context = use_context::<Signal<ApplicationContext>>();
    let _resource = use_resource(use_reactive!(|(company_id, role_id)| {
        let store = store.clone();
        let mut context = context.clone();
        async move {
            if let Some(company_id) = company_id {
                if context().get_company().map(|c| c.id) != Some(company_id) {
                    let company = store
                        .recall_by_id(company_id)
                        .await
                        .expect("Could not set company");
                    context.set(context().set_company(company));
                }
            }
            // Role _must_ be set after company, though could load both at the same time
            if let Some(role_id) = role_id {
                if context().get_role().map(|c| c.id) != Some(role_id) {
                    let role = store
                        .recall_by_id(role_id)
                        .await
                        .expect("Could not set role");
                    context.set(context().set_role(role).expect("Couldn't set role"));
                }
            }
        }
    }));

    rsx! {
        p {
            "Company ID: "
            {company_id.map(|id| id.to_string())}
            " - Role ID: "
            {role_id.map(|id| id.to_string())}
        }

        div { id: "home", class: "flex",

            section { id: "left-home",
                CompanyList { }
                RoleList { }
            }

            Details {}
        }
    }
}

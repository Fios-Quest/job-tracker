use crate::router::DetailsView;
use crate::{CompanyList, Details, RoleList, StoreType};
use dioxus::prelude::*;
use storage::prelude::RecallById;
use storage::ApplicationContext;
use uuid::Uuid;

#[component]
pub fn Home(company_id: Option<Uuid>, role_id: Option<Uuid>, view: Option<DetailsView>) -> Element {
    let store = use_context::<StoreType>();
    let mut context = use_context::<Signal<ApplicationContext>>();
    let _resource = use_resource(use_reactive!(|(company_id, role_id)| {
        let store = store.clone();
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
        div { id: "home", class: "flex",

            section { class: "mr-4 mt-0",
                CompanyList {}
                RoleList {}
            }

            Details { view }
        }
    }
}

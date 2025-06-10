use crate::router::Route;
use crate::{CompanyDetails, CompanyList, Navbar, RoleDescription, RoleList, StoreType};
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
    let _resource = use_resource(move || async move {
        let store = use_context::<StoreType>();
        let mut context = use_context::<ApplicationContext>();
        if let Some(company_id) = company_id {
            let company = store
                .recall_by_id(company_id)
                .await
                .expect("Could not set company");
            context.set_company(company);
        }
        // Role _must_ be set after company, though could load both at the same time
        if let Some(role_id) = role_id {
            let role = store
                .recall_by_id(role_id)
                .await
                .expect("Could not set role");
            context.set_role(role).expect("Couldn't set role");
        }
    });

    rsx! {
        div { id: "home", class: "flex",

            section { id: "left-home",
                CompanyList {}
                RoleList { company_id }
            }

            Navbar {
                Link { class: "hover:underline", to: Route::Help {}, "Company Details" }
                Link { class: "hover:underline", to: Route::Help {}, "Role Details" }
                Link { class: "hover:underline", to: Route::Help {}, "Interview Details" }
                Link { class: "hover:underline", to: Route::Help {}, "Questions" }
            }

            section {
                CompanyDetails { company_id }
            }

            section { id: "role-information",
                RoleDescription { role_id }
            }
        }
    }
}

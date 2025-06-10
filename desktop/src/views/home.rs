use dioxus::prelude::*;
use storage::ApplicationContext;
use ui::{CompanyDetails, CompanyList, FlagList, RoleDescription, RoleList};

#[component]
pub fn Home() -> Element {
    let application_context = use_context::<Signal<ApplicationContext>>();
    let company_id = application_context().get_company_id();
    let role = application_context().get_role().cloned();

    rsx! {
        div { id: "home", class: "grid grid-cols-2",

            section { id: "left-home",
                CompanyList {}
                RoleList { company_id }
            }

            section {
                CompanyDetails { company_id }
            }

            section { id: "role-information",
                RoleDescription { role }
            }
        }
    }
}

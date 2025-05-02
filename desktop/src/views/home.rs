use dioxus::prelude::*;
use storage::ApplicationContext;
use ui::{CompanyList, FlagList, RoleDescription, RoleList};

#[component]
pub fn Home() -> Element {
    let application_context = use_context::<Signal<ApplicationContext>>();
    let company_id = application_context().get_company_id();

    rsx! {
        div { id: "Home",

            section { id: "left-home",
                CompanyList {}
                RoleList { company_id }
                FlagList { company_id }
            }

            section { id: "role-information", RoleDescription {} }
        }
    }
}

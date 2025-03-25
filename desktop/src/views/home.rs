use dioxus::prelude::*;
use storage::ApplicationContext;
use ui::{CompanyList, EmptyRolesList, RolesList};

#[component]
pub fn Home() -> Element {
    let application_context = use_context::<Signal<ApplicationContext>>();
    let company_id = application_context().get_company_id();
    let role_id = application_context().get_role_id();

    rsx! {
        CompanyList { }
        if let Some(company_id) = company_id {
            RolesList { company_id }
        } else {
            EmptyRolesList {}
        }
        if let Some(role_id) = role_id {
            "{role_id}"
        } else {
            "no role selected"
        }

    }
}

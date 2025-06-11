mod empty_role_list;
mod populated_role_list;
mod role_list_item;

use dioxus::prelude::*;
use empty_role_list::EmptyRoleList;
use populated_role_list::PopulatedRoleList;
use storage::ApplicationContext;

#[component]
pub fn RoleList() -> Element {
    let company = use_context::<Signal<ApplicationContext>>()().get_company();

    match company {
        Some(company) => rsx! {
            PopulatedRoleList { company }
        },
        None => rsx! {
            EmptyRoleList {}
        },
    }
}

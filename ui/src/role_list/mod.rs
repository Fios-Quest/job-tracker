mod empty_role_list;
mod populated_role_list;
mod role_list_item;

use dioxus::prelude::*;
use empty_role_list::EmptyRoleList;
use populated_role_list::PopulatedRoleList;
use uuid::Uuid;

#[component]
pub fn RoleList(company_id: Option<Uuid>) -> Element {
    match company_id {
        Some(company_id) => rsx! { PopulatedRoleList { company_id } },
        None => rsx! { EmptyRoleList {} },
    }
}

use super::role_information::RoleDescription;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn RoleDetails(role: Arc<Role>) -> Element {
    rsx! {
        h2 { "{role.get_name()}" }
        RoleDescription { role }
    }
}

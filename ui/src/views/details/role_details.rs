use crate::RoleDescription;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{HasName, Role};

#[component]
pub fn RoleDetails(role: Arc<Role>) -> Element {
    rsx! {
        h2 { class: "text-slate-200 text-3xl", "{role.get_name()} Details" }
        RoleDescription { role }
    }
}

use crate::helpers::edit_with_form;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{Role, RoleFieldName};

#[component]
pub fn EditRoleDescription(role: Arc<Role>, callback: Callback<Role>) -> Element {
    rsx! {
        form { onsubmit: edit_with_form(use_context::<StoreType>(), role.clone(), callback),
            textarea { name: RoleFieldName::Description.name(), "{role.description}" }
            input { r#type: "submit" }
        }
    }
}

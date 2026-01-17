use crate::helpers::edit_with_form;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{Company, CompanyFieldName};

#[component]
pub fn EditCompanyName(company: Arc<Company>, callback: Callback<Company>) -> Element {
    rsx! {
        form { onsubmit: edit_with_form(use_context::<StoreType>(), company.clone(), callback),
            input {
                id: "{company.id}",
                r#type: "text",
                name: CompanyFieldName::Name.name(),
                value: "{company.name}",
            }
            input { r#type: "submit" }
        }
    }
}

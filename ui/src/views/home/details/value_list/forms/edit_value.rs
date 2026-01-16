use crate::helpers::edit_with_form;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{Value, ValueFieldName};

#[component]
pub fn EditValue(value: Arc<Value>, callback: Callback<Value>) -> Element {
    rsx! {
        form { onsubmit: edit_with_form(use_context::<StoreType>(), value.clone(), callback),
            input { name: ValueFieldName::Name.name(), value: "{value.name}" }
            textarea {
                name: ValueFieldName::Description.name(),
                value: "{value.description}",
            }
            input { r#type: "submit" }
        }
    }
}

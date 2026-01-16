use crate::helpers::edit_with_form;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{Flag, FlagColor, FlagFieldName};

#[component]
pub fn EditFlag(flag: Arc<Flag>, callback: Callback<Flag>) -> Element {
    rsx! {
        form { onsubmit: edit_with_form(use_context::<StoreType>(), flag.clone(), callback),
            select { name: FlagFieldName::FlagColor.name(),
                option { selected: flag.flag_color == FlagColor::Red, value: "red", "🚩 Red" }
                option {
                    selected: flag.flag_color == FlagColor::Green,
                    value: "green",
                    "💚 Green"
                }
            }
            input { name: FlagFieldName::Name.name(), value: "{flag.name}" }
            input { r#type: "submit" }
        }
    }
}

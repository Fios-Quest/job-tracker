use dioxus::prelude::*;
use storage::prelude::*;

#[component]
pub fn ValueListItem(value: Value) -> Element {
    rsx! {
        li {
            header { "{value.name}" }
            "{value.description}"
        }
    }
}

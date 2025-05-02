use dioxus::prelude::*;

#[component]
pub fn RoleDescription() -> Element {
    rsx! {
        h3 { "Role Description" }
        textarea {}
    }
}

use dioxus::prelude::*;

#[component]
pub fn EmptyRoleList() -> Element {
    rsx! {
        div { id: "roles" }

        h3 { "Roles" }

        p { "Select a company to see Roles" }
    }
}

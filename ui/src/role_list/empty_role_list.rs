use dioxus::prelude::*;

#[component]
pub fn EmptyRoleList() -> Element {
    rsx! {
        div { id: "roles",

            h3 { class: "text-2xl", "Roles" }

            p { "Select a company to see Roles" }
        }
    }
}

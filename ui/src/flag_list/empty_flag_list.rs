use dioxus::prelude::*;

#[component]
pub fn EmptyFlagList() -> Element {
    rsx! {
        div { id: "flags" }

        h3 { "Flags" }

        p { "Select a company to see Flags" }
    }
}
